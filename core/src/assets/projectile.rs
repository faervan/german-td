use crate::{
    assets::{AssetLoadedHook, AssetNameExt, RonAsset, RonAssetLoader},
    prelude::*,
};

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.init_asset::<ProjectileDefinition>();
        app.register_asset_loader(RonAssetLoader::<ProjectileAsset>::new());
        app.load_folder("projectiles");

        app.init_library::<ProjectileDefinition, STATE>(loading_state);
    }
}

#[derive(TypePath, Debug, Serialize, Deserialize)]
struct ProjectileAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    pub damage: f32,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct ProjectileDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub icon: Handle<Image>,
    pub damage: f32,
}

impl RonAsset for ProjectileAsset {
    type Asset = ProjectileDefinition;
    const EXTENSION: &str = "projectile";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        ProjectileDefinition {
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
