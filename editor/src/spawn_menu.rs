use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContext, EguiPrimaryContextPass};
use egui::Ui;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<MenuActive>();

    app.add_systems(
        EguiPrimaryContextPass,
        spawn_menu.run_if(in_state(MenuActive(true))),
    );
    app.add_systems(
        Update,
        (|mut state: ResMut<NextState<MenuActive>>| {
            state.set(MenuActive(true));
        })
        .run_if(
            in_state(MenuActive(false))
                .and(input_pressed(KeyCode::ControlLeft))
                .and(input_just_pressed(KeyCode::KeyA)),
        ),
    );
    app.add_systems(
        Update,
        (|mut state: ResMut<NextState<MenuActive>>| {
            state.set(MenuActive(false));
        })
        .run_if(in_state(MenuActive(true)).and(input_just_pressed(KeyCode::Escape))),
    );
}

#[derive(SubStates, Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[source(State = State::Editor)]
struct MenuActive(bool);

fn spawn_menu(world: &mut World) {
    let mut query = world.query_filtered::<&Window, With<PrimaryWindow>>();
    let Some(cursor_pos) = query.single(world).unwrap().cursor_position() else {
        return;
    };

    let mut query = world.query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>();
    let mut egui_context = query.single_mut(world).unwrap().clone();

    egui::Window::new("Spawn menu")
        .default_size((300., 500.))
        .default_pos(cursor_pos.as_ref())
        .title_bar(false)
        .show(egui_context.get_mut(), |ui| {
            ui.vertical(|ui| {
                spawn_enemy(world, ui);
                spawn_tower(world, ui);
                spawn_waypoint(world, ui);
                spawn_plot(world, ui);
            });
        });
}

fn spawn_enemy(world: &mut World, ui: &mut Ui) {
    if ui.button("Spawn Enemy").clicked() {
        let definition =
            world.resource::<AssetLibrary<EnemyDefinition>>().entries["Knight"].clone();
        let position = world
            .query_filtered::<&Transform, With<EditorCursor>>()
            .single(world)
            .unwrap()
            .translation;
        world.write_message(SpawnEnemy {
            position,
            definition,
            waypoints: Arc::new(vec![]),
        });

        close_menu(world);
    }
}

fn spawn_tower(world: &mut World, ui: &mut Ui) {
    if ui.button("Spawn Tower").clicked() {
        let definition =
            world.resource::<AssetLibrary<TowerDefinition>>().entries["Bow Turret"].clone();
        let position = world
            .query_filtered::<&Transform, With<EditorCursor>>()
            .single(world)
            .unwrap()
            .translation;
        world.write_message(SpawnTower {
            position,
            definition,
        });

        close_menu(world);
    }
}

fn spawn_waypoint(world: &mut World, ui: &mut Ui) {
    if ui.button("Spawn Waypoint").clicked() {
        world.write_message(SpawnWaypoint { position: None });

        close_menu(world);
    }
}

fn spawn_plot(world: &mut World, ui: &mut Ui) {
    if ui.button("Spawn TowerPlot").clicked() {
        world.write_message(SpawnTowerPlot { position: None });

        close_menu(world);
    }
}

fn close_menu(world: &mut World) {
    world
        .resource_mut::<NextState<MenuActive>>()
        .set(MenuActive(false));
}
