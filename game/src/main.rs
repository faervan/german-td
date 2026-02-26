mod camera;
mod dev_tools;
mod enemy;
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
    app.add_plugins((
        default_plugins(AppState::Loading, AppState::Game),
        camera::plugin,
        enemy::plugin,
        dev_tools::plugin,
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
    app.add_systems(OnEnter(AppState::Game), log_loaded_towers);
    app.add_systems(Update, enemy_ctrl.run_if(in_state(AppState::Game)));

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

fn demo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    enemy_lib: EnemyLibrary,
    tower_lib: TowerLibrary,
    mut enemy_spawner: MessageWriter<SpawnEnemy>,
    mut tower_spawner: MessageWriter<SpawnTower>,
) {
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)))),
        MeshMaterial3d(materials.add(Color::Srgba(Srgba {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }))),
    ));

    // Enemy
    enemy_spawner.write(SpawnEnemy {
        position: Vec3::new(0., 0.5, 0.),
        definition: enemy_lib.entries["Knight"].clone(),
    });

    // "Tower"
    tower_spawner.write(SpawnTower {
        position: Vec3::new(0., 0., -15.),
        definition: tower_lib.entries["Bow Turret"].clone(),
    });
}

fn enemy_ctrl(input: Res<ButtonInput<KeyCode>>, mut controllers: Query<&mut EnemyController>) {
    if input.just_pressed(KeyCode::KeyH) {
        for mut controller in &mut controllers {
            if !controller.attack() {
                warn!("Already attacking!");
            }
        }
    }

    if input.just_pressed(KeyCode::KeyJ) {
        for mut controller in &mut controllers {
            if !controller.start_moving() {
                warn!("Already moving!");
            }
        }
    }

    if input.just_pressed(KeyCode::KeyK) {
        for mut controller in &mut controllers {
            if !controller.stop_moving() {
                warn!("Not moving currently!");
            }
        }
    }
}
