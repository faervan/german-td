mod camera;
mod prelude;

use german_td_core::{asset_plugin, default_plugins};

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    // Bevy config
    app.add_plugins(DefaultPlugins.set(asset_plugin()).set(WindowPlugin {
        primary_window: Some(Window {
            title: "German TD".into(),
            name: Some("german_td_game".into()),
            ..Default::default()
        }),
        ..Default::default()
    }));

    // Our plugins
    app.add_plugins((default_plugins(AppState::Loading), camera::plugin));

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
    app.add_systems(OnEnter(AppState::Game), log_loaded_towers);

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

fn log_loaded_enemies(enemy_lib: EnemyLibrary) {
    info!(
        "enemies loaded:\n{}",
        enemy_lib
            .entries
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn log_loaded_towers(tower_lib: TowerLibrary) {
    info!(
        "towers loaded:\n{}",
        tower_lib
            .entries
            .keys()
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn demo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)))),
        MeshMaterial3d(materials.add(Color::Srgba(Srgba {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }))),
    ));
}
