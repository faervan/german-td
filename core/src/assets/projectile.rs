use crate::{
    assets::{AssetLoadedHook, AssetNameExt, RonAsset, RonAssetLoader},
    prelude::*,
};

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.init_asset::<ProjectileDefinition>();
        app.register_asset_loader(RonAssetLoader::<ProjectileAsset>::default());
        app.load_folder(ProjectileAsset::DIRECTORY);

        app.init_library::<ProjectileDefinition, STATE>(loading_state);
    }
}

#[derive(TypePath, Default, Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "editor", serde(default))]
/// New fields need to be manually added to the projectile editor tab
pub struct ProjectileAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    pub damage: f32,
}

#[cfg_attr(feature = "editor", derive(Deref, DerefMut))]
#[derive(Asset, Reflect, Default, Debug)]
#[reflect(Asset)]
pub struct ProjectileDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub icon: Handle<Image>,
    pub damage: f32,
    #[cfg(feature = "editor")]
    #[reflect(ignore)]
    #[deref]
    pub asset: ProjectileAsset,
}

#[cfg(feature = "editor")]
impl ProjectileDefinition {
    /// Returns (name, serialized asset) on success
    pub fn serialize(&mut self) -> Result<(String, String), ron::Error> {
        use ron::ser::PrettyConfig;

        ron::ser::to_string_pretty(&self.asset, PrettyConfig::default())
            .map(|s| (self.asset.name.clone(), s))
    }
}

impl RonAsset for ProjectileAsset {
    type Asset = ProjectileDefinition;
    const DIRECTORY: &str = "projectiles";
    const EXTENSION: &str = "projectile";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        ProjectileDefinition {
            #[cfg(feature = "editor")]
            asset: self.clone(),
            name: self.name,
            gltf: context.load(self.gltf),
            scene: Default::default(),
            icon: context.load(self.icon),
            damage: self.damage,
        }
    }
}

impl AssetNameExt for ProjectileDefinition {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl AssetLoadedHook for ProjectileDefinition {
    fn on_loaded_hook(&mut self, world: &mut World) {
        let gltf = world.resource::<Assets<Gltf>>().get(&self.gltf).unwrap();
        self.scene = gltf.default_scene.clone().expect("Missing default scene");
    }
}
