use bevy::{
    input::common_conditions::input_just_pressed,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn, cursor_grab))
        .add_systems(
            Update,
            (move_camera_key, move_camera_edge, move_camera_grab),
        )
        .add_systems(
            Update,
            toggle_cursor.run_if(input_just_pressed(KeyCode::Escape)),
        );
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct PrimaryCamera;

// Spawn camera at default position looking at origin
fn spawn(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        PrimaryCamera,
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 100.0,
            z: 100.0,
        })
        .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

const UP_KEYS: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
const DOWN_KEYS: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
const LEFT_KEYS: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
const RIGHT_KEYS: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];

const VELOCITY: f32 = 50.0;

// Key based camera movement
// TODO: Replace with physics based movement and unify with other movement?
fn move_camera_key(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut Transform, With<PrimaryCamera>>,
) {
    let (mut x, mut z) = (0.0, 0.0);
    if keys.any_pressed(UP_KEYS) {
        z -= 1.0;
    }
    if keys.any_pressed(DOWN_KEYS) {
        z += 1.0;
    }
    if keys.any_pressed(LEFT_KEYS) {
        x -= 1.0;
    }
    if keys.any_pressed(RIGHT_KEYS) {
        x += 1.0;
    }

    let direction = Vec3::new(x, 0.0, z).normalize();

    if direction.length() > 0.0 {
        transform.translation += direction * VELOCITY * time.delta_secs();
    }
}

// edge pan tolerance in percent
const EDGE_PAN_TOLERANCE: f32 = 0.05;

// Mouse edge pan based movement
// Cursor is considered outside to the very right, so we operate on the cached cursor position
// Only runs if cursor mode is confined
// TODO: Change CursorIcon
fn move_camera_edge(
    time: Res<Time>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    cursor_options: Single<&CursorOptions>,
    mut transform: Single<&mut Transform, With<PrimaryCamera>>,
    mut cached_cursor_position: Local<Option<Vec2>>,
) {
    if !matches!(cursor_options.grab_mode, CursorGrabMode::Confined) {
        return;
    }

    if let Some(cursor_position) = primary_window.cursor_position() {
        *cached_cursor_position = Some(cursor_position);
    }

    // This check is kind of dumb, because it should only matter for the local initialization
    if let Some(cursor_position) = *cached_cursor_position {
        let size = primary_window.size();
        let (mut x, mut z) = (0.0, 0.0);

        // Cursor position (0,0) is top left
        if cursor_position.y < (size.y * EDGE_PAN_TOLERANCE) {
            z -= 1.0;
        }
        if cursor_position.y > (size.y * (1.0 - EDGE_PAN_TOLERANCE)) {
            z += 1.0;
        }
        if cursor_position.x < (size.x * EDGE_PAN_TOLERANCE) {
            x -= 1.0;
        }
        if cursor_position.x > (size.x * (1.0 - EDGE_PAN_TOLERANCE)) {
            x += 1.0;
        }

        let direction = Vec3::new(x, 0.0, z).normalize();

        if direction.length() > 0.0 {
            transform.translation += direction * VELOCITY * time.delta_secs();
        }
    }
}

// Mouse grab based movement
// Middle mouse button down saved current cursor position
// current cursor position compared to saved cursor position determines direction (reversed?)
// Middle button up resets saved current cursor position
// TODO: Change CursorIcon
// TODO: Visualize grab_cursor_position
fn move_camera_grab(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    primary_window: Single<&Window, With<PrimaryWindow>>,
    cursor_options: Single<&CursorOptions>,
    mut transform: Single<&mut Transform, With<PrimaryCamera>>,
    mut cached_cursor_position: Local<Option<Vec2>>,
    mut grab_cursor_position: Local<Option<Vec2>>,
) {
    if !matches!(cursor_options.grab_mode, CursorGrabMode::Confined) {
        return;
    }

    if let Some(cursor_position) = primary_window.cursor_position() {
        *cached_cursor_position = Some(cursor_position);
    }

    if mouse.just_released(MouseButton::Middle) {
        *grab_cursor_position = None;
    }
    if mouse.just_pressed(MouseButton::Middle) {
        *grab_cursor_position = *cached_cursor_position;
    }

    if let Some(grab_cursor_position) = *grab_cursor_position
        && let Some(cursor_position) = *cached_cursor_position
    {
        let delta = grab_cursor_position - cursor_position;

        let direction = Vec3::new(delta.x, 0.0, delta.y).normalize();

        if direction.length() > 0.0 {
            transform.translation += direction * VELOCITY * time.delta_secs();
        }
    }
}

fn cursor_grab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::Confined;
}

fn cursor_ungrab(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::None;
}

fn toggle_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    if let CursorGrabMode::None = cursor_options.grab_mode {
        cursor_options.grab_mode = CursorGrabMode::Confined;
    } else {
        cursor_options.grab_mode = CursorGrabMode::None;
    }
}
