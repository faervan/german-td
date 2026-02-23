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
    name: String,
}

#[derive(Asset, Reflect, Debug)]
#[reflect(Asset)]
pub struct EnemyDefinition {
    name: String,
}

impl RonAsset for EnemyAsset {
    type Asset = EnemyDefinition;
    const EXTENSION: &str = "enemy";

    async fn load_dependencies(self, _context: &mut bevy::asset::LoadContext<'_>) -> Self::Asset {
        EnemyDefinition { name: self.name }
    }
}

impl AssetNameExt for EnemyDefinition {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
