#![feature(str_as_str)]

pub mod prelude;
use prelude::*;

pub mod assets;
mod components;
mod enemy;
mod events;
mod maps;
mod physics_layers;
mod projectile;
mod resources;
pub mod scripting;
mod skein_spawners;
mod tower;
pub mod utils;

pub fn default_plugins<STATE: States + Copy>(
    loading_state: STATE,
    game_state: STATE,
) -> impl Plugin {
    move |app: &mut App| {
        app.add_plugins(MeshPickingPlugin);

        app.add_plugins(PhysicsPlugins::default());
        app.add_plugins(PhysicsPickingPlugin);
        app.insert_resource(PhysicsPickingSettings {
            require_markers: true,
        });

        app.add_plugins(bevy_skein::SkeinPlugin::default());

        app.add_plugins((
            assets::plugin(loading_state),
            utils::delayed_despawn::plugin,
            utils::value_animation::plugin,
            utils::billboard::plugin(game_state),
            utils::ui_deselection::plugin(game_state),
            skein_spawners::plugin,
            enemy::plugin(game_state),
            maps::plugin(game_state),
            tower::plugin(game_state),
            projectile::plugin(game_state),
            events::plugin,
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
