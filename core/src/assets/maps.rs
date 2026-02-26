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

#[derive(TypePath, Debug, Serialize, Deserialize)]
struct MapAsset {
    name: String,
    gltf: String,
    icon: String,
    waypoints: Vec<Vec3>,
    paths: Vec<EnemyPathAsset>,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct MapDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub icon: Handle<Image>,
    /// A waypoint is a position in 3d space, and the primitive of a path
    pub waypoints: Vec<Vec3>,
    pub paths: Vec<EnemyPath>,
    pub waves: usize,
    #[reflect(ignore)]
    asset: Option<MapAsset>,
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
    spawns: Vec<Vec<(Timer, String)>>,
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
            asset: Some(self),
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

        let asset = self.asset.take().unwrap();
        let mut enemies = world.resource_mut::<Assets<EnemyDefinition>>();
        let mut enemy_ids: HashMap<_, _> = asset
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

        self.paths = asset
            .paths
            .into_iter()
            .map(|path| EnemyPath {
                waypoints: path.waypoints,
                spawner: EnemySpawnPoint {
                    spawns: path
                        .spawner
                        .spawns
                        .into_iter()
                        .map(|wave| {
                            wave.into_iter()
                                .flat_map(|(timer, name)| {
                                    Some((
                                        timer,
                                        enemies.get_strong_handle(
                                            enemy_ids[&name].inspect_none(|| {
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
