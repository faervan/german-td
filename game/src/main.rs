mod camera;
mod dev_tools;
mod game_over;
mod gold;
mod prelude;
mod waves;
use german_td_core::{asset_plugin, default_plugins};

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    let present_mode = match std::env::var("WAYLAND_DISPLAY").is_ok() {
        true => bevy::window::PresentMode::Mailbox,
        false => bevy::window::PresentMode::AutoVsync,
    };

    // Bevy config
    app.add_plugins(DefaultPlugins.set(asset_plugin()).set(WindowPlugin {
        primary_window: Some(Window {
            title: "German TD".into(),
            name: Some("german_td_game".into()),
            present_mode,
            ..Default::default()
        }),
        ..Default::default()
    }));
    // Avoid needing to add [`Picking::IGNORE`] to 90% of UI nodes
    app.insert_resource(UiPickingSettings {
        require_markers: true,
    });

    // Our plugins
    app.add_plugins((
        default_plugins(AppState::Loading, AppState::Game),
        camera::plugin,
        dev_tools::plugin,
        game_over::plugin,
        waves::plugin,
        gold::plugin,
    ));

    // Our states
    app.init_state::<AppState>();

    // Our systems
    app.add_systems(
        Update,
        set_game_state.run_if(in_state(AppState::Loading).and(all_assets_loaded)),
    );

    // Test systems
    app.add_systems(OnEnter(AppState::Game), demo);
    app.add_systems(OnEnter(AppState::Game), log_loaded_enemies);
    app.add_systems(OnEnter(AppState::Game), log_loaded_maps);
    app.add_systems(OnEnter(AppState::Game), log_loaded_towers);
    app.add_systems(OnEnter(AppState::Game), log_loaded_projectiles);

    app.run();
}

#[derive(States, Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Game,
}

fn set_game_state(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Game);
}

fn log_loaded_enemies(enemy_lib: EnemyLibrary, enemies: Res<Assets<EnemyDefinition>>) {
    info!(
        "enemies loaded:\n{}",
        enemy_lib
            .entries
            .values()
            .map(|v| format!("{:#?}", enemies.get(v)))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn log_loaded_maps(map_lib: MapLibrary, maps: Res<Assets<MapDefinition>>) {
    info!(
        "maps loaded:\n{}",
        map_lib
            .entries
            .values()
            .map(|v| format!("{:#?}", maps.get(v)))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn log_loaded_towers(tower_lib: TowerLibrary, towers: Res<Assets<TowerDefinition>>) {
    info!(
        "towers loaded:\n{}",
        tower_lib
            .entries
            .values()
            .map(|v| format!("{:#?}", towers.get(v)))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn log_loaded_projectiles(
    projectile_lib: ProjectileLibrary,
    projectiles: Res<Assets<ProjectileDefinition>>,
) {
    info!(
        "projectile loaded:\n{}",
        projectile_lib
            .entries
            .values()
            .map(|v| format!("{:#?}", projectiles.get(v)))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn demo(map_lib: MapLibrary, mut map_spawner: MessageWriter<SpawnMap>) {
    // Map
    map_spawner.write(SpawnMap {
        definition: map_lib.entries["First"].clone(),
    });
}
