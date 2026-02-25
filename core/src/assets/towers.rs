use crate::{
    assets::{AssetNameExt, RonAsset, RonAssetLoader},
    prelude::*,
};

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.load_folder("towers");

        app.init_asset::<TowerDefinition>();
        app.register_asset_loader(RonAssetLoader::<TowerAsset>::new());
        app.init_library::<TowerDefinition, STATE>(loading_state);
    }
}

#[derive(TypePath, Debug, Serialize, Deserialize)]
struct TowerAsset {
    pub name: String,
    pub gltf: String,
    pub icon: String,
    pub damage: f32,
    pub cost: f32,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct TowerDefinition {
    pub name: String,
    pub gltf: Handle<Gltf>,
    pub icon: Handle<Image>,
    pub damage: f32,
    pub cost: f32,
}

impl RonAsset for TowerAsset {
    type Asset = TowerDefinition;
    const EXTENSION: &str = "tower";

    async fn load_dependencies(self, context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        TowerDefinition {
            name: self.name,
            gltf: context.load(self.gltf),
            icon: context.load(self.icon),
            damage: self.damage,
            cost: self.cost,
        }
    }
}

impl AssetNameExt for TowerDefinition {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
