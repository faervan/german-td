use crate::{
    assets::{AssetNameExt, RonAsset, RonAssetLoader},
    prelude::*,
};

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.load_folder("enemies");

        app.init_asset::<EnemyDefinition>();
        app.register_asset_loader(RonAssetLoader::<EnemyAsset>::new());
        app.init_library::<EnemyDefinition, STATE>(loading_state);
    }
}

#[derive(TypePath, Debug, Serialize, Deserialize)]
struct EnemyAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    pub damage: f32,
    pub walk_speed: f32,
    pub health: f32,
    pub drop: f32,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct EnemyDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub icon: Handle<Image>,
    pub damage: f32,
    pub walk_speed: f32,
    pub health: f32,
    pub drop: f32,
}

impl RonAsset for EnemyAsset {
    type Asset = EnemyDefinition;
    const EXTENSION: &str = "enemy";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        EnemyDefinition {
            name: self.name,
            gltf: context.load(self.gltf),
            icon: context.load(self.icon),
            damage: self.damage,
            walk_speed: self.walk_speed,
            health: self.health,
            drop: self.drop,
        }
    }
}

impl AssetNameExt for EnemyDefinition {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
