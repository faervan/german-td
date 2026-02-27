use bevy::input::mouse::AccumulatedMouseMotion;
use german_td_core::utils::{grab_cursor, ungrab_cursor};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(State::Editor), setup);
    app.add_systems(
        Update,
        (movement, rotation.run_if(input_pressed(MouseButton::Right)))
            .run_if(in_state(State::Editor)),
    );

    app.add_systems(
        Update,
        grab_cursor.run_if(input_just_pressed(MouseButton::Right)),
    );
    app.add_systems(
        Update,
        ungrab_cursor.run_if(input_just_released(MouseButton::Right)),
    );
}

#[derive(Component)]
struct EditorCamera;

fn setup(mut commands: Commands) {
    info!(
        "\nCamera controls:\n\
        WASD, Shift/Space to move\n\
        Hold right click + move cursor to rotate\n\n\
        Keybinds:\n\
        <CR>q to quit"
    );

    commands.spawn((
        EditorCamera,
        Name::new("EditorCamera"),
        Camera3d::default(),
        Transform::default(),
    ));
}

const MOVE_SPEED: f32 = 50.;
fn movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&mut Transform, With<EditorCamera>>,
) {
    for mut transform in query {
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) {
            direction += Vec3::NEG_Z;
        }
        if input.pressed(KeyCode::KeyA) {
            direction += Vec3::NEG_X;
        }
        if input.pressed(KeyCode::KeyS) {
            direction += Vec3::Z;
        }
        if input.pressed(KeyCode::KeyD) {
            direction += Vec3::X;
        }
        if input.pressed(KeyCode::Space) {
            direction += Vec3::Y;
        }
        if input.pressed(KeyCode::ShiftLeft) {
            direction += Vec3::NEG_Y;
        }

        let rotation = transform.rotation;
        transform.translation += rotation * direction * MOVE_SPEED * time.delta_secs();
    }
}

fn rotation(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<&mut Transform, With<EditorCamera>>,
) {
    let Ok(mut camera) = query.single_mut() else {
        return;
    };

    let delta = mouse_motion.delta;
    if delta != Vec2::ZERO {
        let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

        let yaw = yaw + delta.x * -0.001;
        let pitch = (pitch + delta.y * -0.001).clamp(-0.9, 0.9);

        camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
