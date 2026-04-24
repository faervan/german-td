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

#[derive(Default, Debug, Clone, Copy, Reflect, Serialize, Deserialize)]
pub enum DamageTypeAsset {
    #[default]
    Single,
    Area {
        radius: f32,
    },
}

#[derive(Debug, Reflect, Clone, Copy)]
pub enum DamageType {
    Single { target: Entity },
    Area { radius: f32, target_pos: Vec3 },
}

#[derive(TypePath, Default, Debug, Clone, Serialize, Deserialize)]
pub struct TowerAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    pub projectile: String,
    pub damage_factor: f32,
    pub attack_duration_ms: u64,
    pub range: f32,
    pub cost: usize,
    pub damage_type: DamageTypeAsset,
    pub upgrades: Vec<String>,
    pub starter_tower: bool,
}

#[cfg_attr(feature = "editor", derive(Default, Deref, DerefMut))]
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
    pub damage_type: DamageTypeAsset,
    pub upgrades: Vec<Handle<TowerDefinition>>,
    /// Marks this tower as buildable directly on an empty plot. We probably always want this to be
    /// `false` for upgrade towers.
    pub starter_tower: bool,
    #[cfg(feature = "editor")]
    #[reflect(ignore)]
    #[deref]
    pub asset: TowerAsset,
}

#[cfg(feature = "editor")]
impl TowerDefinition {
    /// TODO! Maybe just make [`TowerAsset`] public for the editor crate instead
    pub fn path(name: &str) -> PathBuf {
        TowerAsset::path(name)
    }

    /// Returns (name, serialized asset) on success
    pub fn serialize(&mut self) -> Result<(String, String), ron::Error> {
        use ron::ser::PrettyConfig;

        ron::ser::to_string_pretty(&self.asset, PrettyConfig::default())
            .map(|s| (self.asset.name.clone(), s))
    }
}

impl RonAsset for TowerAsset {
    type Asset = TowerDefinition;
    const DIRECTORY: &str = "towers";
    const EXTENSION: &str = "tower";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        TowerDefinition {
            #[cfg(feature = "editor")]
            asset: self.clone(),
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
            upgrades: self
                .upgrades
                .into_iter()
                .map(|name| context.load(TowerAsset::path(&name)))
                .collect(),
            starter_tower: self.starter_tower,
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
