pub mod prelude;
use prelude::*;

mod assets;
mod components;
mod enemy;
mod maps;
mod projectile;
mod skein_spawners;
mod tower;
pub mod utils;

pub fn default_plugins<STATE: States + Copy>(
    loading_state: STATE,
    game_state: STATE,
) -> impl Plugin {
    move |app: &mut App| {
        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins(bevy_skein::SkeinPlugin::default());

        app.add_plugins((
            assets::plugin(loading_state),
            utils::delayed_despawn::plugin,
            skein_spawners::plugin,
            enemy::plugin(game_state),
            maps::plugin(game_state),
            tower::plugin(game_state),
            projectile::plugin(game_state),
        ));
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
