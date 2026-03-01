use ron::ser::PrettyConfig;

use crate::{
    assets::{AssetLoadedHook, AssetNameExt, RonAsset, RonAssetLoader},
    prelude::*,
};

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.load_folder("maps");

        app.init_asset::<MapDefinition>();
        app.register_asset_loader(RonAssetLoader::<MapAsset>::new());
        app.init_library::<MapDefinition, STATE>(loading_state);
    }
}

#[derive(TypePath, Default, Debug, Serialize, Deserialize)]
struct MapAsset {
    name: String,
    gltf: String,
    icon: String,
    waypoints: Vec<Vec3>,
    paths: Vec<EnemyPathAsset>,
    tower_plots: Vec<Vec3>,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
/// TODO! Some fields like `self.name` can be replaced with `self.name()` pointing to the inner asset
pub struct MapDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub icon: Handle<Image>,
    /// A waypoint is a position in 3d space, and the primitive of a path
    pub waypoints: Vec<Vec3>,
    pub paths: Vec<EnemyPath>,
    pub waves: usize,
    /// Positions at which towers can be placed
    pub tower_plots: Vec<Vec3>,
    #[reflect(ignore)]
    asset: MapAsset,
}

impl MapDefinition {
    /// Returns (name, serialized asset) on success
    pub fn serialize(
        &mut self,
        enemies: &Assets<EnemyDefinition>,
    ) -> Result<(String, String), ron::Error> {
        self.asset.waypoints = self.waypoints.clone();
        self.asset.tower_plots = self.tower_plots.clone();
        self.asset.paths = self
            .paths
            .iter()
            .map(|path| EnemyPathAsset {
                waypoints: path.waypoints.clone(),
                spawner: EnemySpawnPointAsset {
                    spawns: path
                        .spawner
                        .spawns
                        .iter()
                        .map(|wave| {
                            wave.iter()
                                .filter_map(|(timer, enemy_handle)| {
                                    Some((
                                        timer.duration().as_millis() as u64,
                                        enemies.get(enemy_handle)?.name.clone(),
                                    ))
                                })
                                .collect()
                        })
                        .collect(),
                },
            })
            .collect();
        ron::ser::to_string_pretty(&self.asset, PrettyConfig::default())
            .map(|s| (self.name.clone(), s))
    }
}

#[derive(Reflect, Debug, Serialize, Deserialize)]
struct EnemyPathAsset {
    waypoints: Vec<usize>,
    spawner: EnemySpawnPointAsset,
}

#[derive(Reflect, Debug)]
pub struct EnemyPath {
    /// Indices into [`MapDefinition.waypoints`]
    pub waypoints: Vec<usize>,
    pub spawner: EnemySpawnPoint,
}

#[derive(Reflect, Debug, Serialize, Deserialize)]
struct EnemySpawnPointAsset {
    /// `u64` represents milliseconds, the String is the name of an enemy
    spawns: Vec<Vec<(u64, String)>>,
}

#[derive(Reflect, Debug)]
pub struct EnemySpawnPoint {
    /// `self.spawns[0]` contains the to-be-spawned definition of the first wave
    /// Spawning happens by taking the entries in the inner [`Vec`] back-to-front, waiting for the
    /// [`Timer`] to finish and spawning the [`EnemyDefinition`].
    pub spawns: Vec<Vec<(Timer, Handle<EnemyDefinition>)>>,
}

impl RonAsset for MapAsset {
    type Asset = MapDefinition;
    const EXTENSION: &str = "map";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        MapDefinition {
            name: self.name.clone(),
            gltf: context.load(&self.gltf),
            scene: Default::default(),
            icon: context.load(&self.icon),
            waypoints: self.waypoints.clone(),
            paths: Default::default(),
            waves: self
                .paths
                .iter()
                .map(|path| path.spawner.spawns.len())
                .max()
                .unwrap_or_default(),
            tower_plots: self.tower_plots.clone(),
            asset: self,
        }
    }
}

impl AssetNameExt for MapDefinition {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl AssetLoadedHook for MapDefinition {
    fn on_loaded_hook(&mut self, world: &mut World) {
        let gltf = world.resource::<Assets<Gltf>>().get(&self.gltf).unwrap();
        self.scene = gltf.default_scene.clone().expect("Missing default scene");

        let mut enemies = world.resource_mut::<Assets<EnemyDefinition>>();
        let mut enemy_ids: HashMap<_, _> = self
            .asset
            .paths
            .iter()
            .flat_map(|p| &p.spawner.spawns)
            .flatten()
            .map(|(_, name)| (name.clone(), None))
            .collect();
        for (id, enemy) in enemies.iter() {
            if let Some(enemy_id) = enemy_ids.get_mut(&enemy.name) {
                *enemy_id = Some(id);
            }
        }

        self.paths = self
            .asset
            .paths
            .iter()
            .map(|path| EnemyPath {
                waypoints: path.waypoints.clone(),
                spawner: EnemySpawnPoint {
                    spawns: path
                        .spawner
                        .spawns
                        .iter()
                        .map(|wave| {
                            wave.iter()
                                .flat_map(|(duration_ms, name)| {
                                    Some((
                                        Timer::new(
                                            Duration::from_millis(*duration_ms),
                                            TimerMode::Once,
                                        ),
                                        enemies.get_strong_handle(
                                            enemy_ids[name].inspect_none(|| {
                                                warn!("No EnemyDefinition with name {name} loaded!")
                                            })?,
                                        )?,
                                    ))
                                })
                                .collect()
                        })
                        .collect(),
                },
            })
            .collect();
    }
}
