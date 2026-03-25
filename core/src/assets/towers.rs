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

#[derive(Debug, Reflect, Serialize, Deserialize)]
pub enum DamageType {
    Single,
    Area { radius: f32 },
}

#[derive(TypePath, Debug, Serialize, Deserialize)]
struct TowerAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    pub projectile: String,
    pub damage_factor: f32,
    pub attack_duration_ms: u64,
    pub range: f32,
    pub cost: usize,
    pub damage_type: DamageType,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct TowerDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub scene: Handle<Scene>,
    pub icon: Handle<Image>,
    pub projectile: Handle<ProjectileDefinition>,
    /// A multiplier to the base damage of the projectiles this tower shoots
    pub damage_factor: f32,
    pub attack_duration: Duration,
    pub range: f32,
    pub cost: usize,
    pub damage_type: DamageType,
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
            projectile: context.load(ProjectileAsset::path(&self.projectile)),
            damage_factor: self.damage_factor,
            attack_duration: Duration::from_millis(self.attack_duration_ms),
            range: self.range,
            cost: self.cost,
            damage_type: self.damage_type,
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
