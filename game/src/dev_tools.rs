//! Development tools for the game. This plugin is only enabled in dev builds.

use std::any::TypeId;

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
    input::common_conditions::{input_just_pressed, input_toggle_active},
};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            enabled: false,
            frame_time_graph_config: FrameTimeGraphConfig {
                enabled: false,
                ..Default::default()
            },
            ..Default::default()
        },
    })
    .add_plugins(EguiPlugin::default())
    .add_plugins(
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, TOGGLE_INSPECTOR_KEY)),
    )
    .add_plugins(PhysicsDebugPlugin)
    .insert_gizmo_config(
        PhysicsGizmos {
            aabb_color: Some(Color::WHITE),
            ..default()
        },
        GizmoConfig {
            enabled: false,
            ..default()
        },
    );

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_DEBUG_UI_KEY)),
    );

    app.add_systems(
        Update,
        toggle_fps_overlay.run_if(input_just_pressed(TOGGLE_FPS_OVERLAY_KEY)),
    );

    app.add_systems(
        Update,
        toggle_physics_gizmos.run_if(input_just_pressed(TOGGLE_PHYSICS_GIZMOS_KEY)),
    );

    app.add_systems(Update, draw_custom_gizmos);
}

const TOGGLE_INSPECTOR_KEY: KeyCode = KeyCode::F1;

const TOGGLE_DEBUG_UI_KEY: KeyCode = KeyCode::F2;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

const TOGGLE_FPS_OVERLAY_KEY: KeyCode = KeyCode::F3;

fn toggle_fps_overlay(mut config: ResMut<FpsOverlayConfig>) {
    config.enabled = !config.enabled;
    config.frame_time_graph_config.enabled = !config.frame_time_graph_config.enabled;
}

const TOGGLE_PHYSICS_GIZMOS_KEY: KeyCode = KeyCode::F4;

fn toggle_physics_gizmos(mut gizmo: ResMut<GizmoConfigStore>) {
    if let Some((config, _)) = gizmo.get_config_mut_dyn(&TypeId::of::<PhysicsGizmos>()) {
        config.enabled = !config.enabled;
    }
}

const TOGGLE_CUSTOM_GIZMOS_KEY: KeyCode = KeyCode::F5;

fn draw_custom_gizmos(
    mut enabled: Local<bool>,
    input: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
    spawners: Query<&Spawner>,
) {
    if input.just_pressed(TOGGLE_CUSTOM_GIZMOS_KEY) {
        *enabled = !*enabled;
    }
    if !*enabled {
        return;
    }
    for spawner in spawners {
        gizmos.cube(
            Transform::from_translation(spawner.position).with_scale(Vec3::splat(5.)),
            Color::srgb(0., 1., 1.),
        );
        let mut prev = None;
        for waypoint in spawner.waypoints.iter() {
            gizmos.cube(
                Transform::from_translation(*waypoint).with_scale(Vec3::splat(5.)),
                Color::srgba(1., 0., 1., 0.5),
            );
            if let Some(prev) = prev.take() {
                gizmos.line(
                    prev + Vec3::Y,
                    *waypoint + Vec3::Y,
                    Color::srgba(1., 0., 1., 0.8),
                );
            }
            prev = Some(*waypoint);
        }
    }
}
