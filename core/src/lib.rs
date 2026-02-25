pub mod prelude;
use prelude::*;

mod assets;
pub mod enemy;

pub fn default_plugins<STATE: States + Copy>(
    loading_state: STATE,
    game_state: STATE,
) -> impl Plugin {
    move |app: &mut App| {
        app.add_plugins((assets::plugin(loading_state), enemy::plugin(game_state)));
    }
}

pub fn asset_plugin() -> AssetPlugin {
    #[cfg(not(feature = "dev"))]
    return AssetPlugin::default();

    #[cfg(feature = "dev")]
    AssetPlugin {
        file_path: "../assets".to_string(),
        ..Default::default()
    }
}
