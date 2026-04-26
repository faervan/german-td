use ron::ser::PrettyConfig;

use crate::{
    assets::{
        AssetLoadedHook, AssetNameExt, RonAsset, RonAssetLoader,
        roto_asset::ScriptAssetLoaderSettings,
    },
    prelude::*,
    scripting::enemy_spawner_runtime,
};

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.init_asset::<MapDefinition>();
        app.register_asset_loader(RonAssetLoader::<MapAsset>::default());
        app.load_folder(MapAsset::DIRECTORY);

        app.init_library::<MapDefinition, STATE>(loading_state);
    }
}

#[derive(TypePath, Default, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "editor", serde(default))]
/// New fields need to be manually added to the map editor tab
pub struct MapAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    /// A waypoint is a position in 3d space, and the primitive of a path
    waypoints: Vec<Vec3>,
    paths: Vec<EnemyPathAsset>,
    /// How many waves will be spawned
    pub waves: usize,
    tower_plots: Vec<Vec3>,
}

#[cfg_attr(feature = "editor", derive(Default, Deref, DerefMut))]
#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct MapDefinition {
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub icon: Handle<Image>,
    pub paths: Vec<EnemyPath>,
    /// Positions at which towers can be placed
    pub tower_plots: Vec<Vec3>,
    #[cfg_attr(feature = "editor", deref)]
    #[reflect(ignore)]
    pub asset: MapAsset,
}

impl MapDefinition {
    pub fn name(&self) -> &String {
        &self.asset.name
    }

    pub fn waypoints(&self) -> &Vec<Vec3> {
        &self.asset.waypoints
    }

    pub fn set_waypoints(&mut self, waypoints: Vec<Vec3>) {
        self.asset.waypoints = waypoints;
    }

    pub fn waves(&self) -> usize {
        self.asset.waves
    }

    /// TODO! Maybe just make [`MapAsset`] public for the editor crate instead
    pub fn path(name: &str) -> PathBuf {
        MapAsset::path(name)
    }

    /// Returns (name, serialized asset) on success
    pub fn serialize(
        &mut self,
        scripts: &Assets<ScriptAsset>,
    ) -> Result<(String, String), ron::Error> {
        self.asset.tower_plots = self.tower_plots.clone();
        self.asset.paths = self
            .paths
            .iter()
            .filter_map(|path| {
                Some(EnemyPathAsset {
                    waypoints: path.waypoints.clone(),
                    spawner: scripts
                        .get(&path.spawner)
                        .inspect_none(|| {
                            warn!("Skipped serializing enemy path because it has no spawn script")
                        })?
                        .file
                        .clone(),
                })
            })
            .collect();
        ron::ser::to_string_pretty(&self.asset, PrettyConfig::default())
            .map(|s| (self.name().clone(), s))
    }
}

#[derive(Reflect, Debug, Serialize, Deserialize)]
struct EnemyPathAsset {
    waypoints: Vec<usize>,
    spawner: String,
}

#[derive(Reflect, Debug)]
pub struct EnemyPath {
    /// Indices into [`MapDefinition.waypoints`]
    pub waypoints: Vec<usize>,
    pub spawner: Handle<ScriptAsset>,
}

impl RonAsset for MapAsset {
    type Asset = MapDefinition;
    const DIRECTORY: &str = "maps";
    const EXTENSION: &str = "map";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        MapDefinition {
            gltf: context.load(&self.gltf),
            scene: Default::default(),
            icon: context.load(&self.icon),
            paths: self
                .paths
                .iter()
                .map(|path| {
                    let spawner_file = path.spawner.clone();
                    EnemyPath {
                        waypoints: path.waypoints.clone(),
                        spawner: context
                            .loader()
                            .with_settings(move |settings: &mut ScriptAssetLoaderSettings| {
                                settings.file = spawner_file.clone();
                                settings.runtime = enemy_spawner_runtime();
                            })
                            .load(&path.spawner),
                    }
                })
                .collect(),
            tower_plots: self.tower_plots.clone(),
            asset: self,
        }
    }
}

impl AssetNameExt for MapDefinition {
    fn get_name(&self) -> String {
        self.name().clone()
    }
}

impl AssetLoadedHook for MapDefinition {
    fn on_loaded_hook(&mut self, world: &mut World) {
        let gltf = world.resource::<Assets<Gltf>>().get(&self.gltf).unwrap();
        self.scene = gltf.default_scene.clone().expect("Missing default scene");
    }
}
