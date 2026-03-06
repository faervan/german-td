use crate::{
    assets::{
        AssetLoadedHook, AssetNameExt, RonAsset, RonAssetLoader, projectile::ProjectileAsset,
    },
    prelude::*,
};

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.init_asset::<TowerDefinition>();
        app.register_asset_loader(RonAssetLoader::<TowerAsset>::default());
        app.load_folder(TowerAsset::DIRECTORY);

        app.init_library::<TowerDefinition, STATE>(loading_state);
    }
}

#[derive(TypePath, Debug, Serialize, Deserialize)]
struct TowerAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    pub damage: f32,
    pub attack_duration_ms: u64,
    pub cost: f32,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct TowerDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub icon: Handle<Image>,
    pub damage: f32,
    pub attack_duration: Duration,
    pub cost: f32,
}

impl RonAsset for TowerAsset {
    type Asset = TowerDefinition;
    const DIRECTORY: &str = "towers";
    const EXTENSION: &str = "tower";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        TowerDefinition {
            name: self.name,
            gltf: context.load(self.gltf),
            scene: Default::default(),
            icon: context.load(self.icon),
            damage: self.damage,
            attack_duration: Duration::from_millis(self.attack_duration_ms),
            cost: self.cost,
        }
    }
}

impl AssetNameExt for TowerDefinition {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

impl AssetLoadedHook for TowerDefinition {
    fn on_loaded_hook(&mut self, world: &mut World) {
        let gltf = world.resource::<Assets<Gltf>>().get(&self.gltf).unwrap();
        self.scene = gltf.default_scene.clone().expect("Missing default scene");
    }
}
