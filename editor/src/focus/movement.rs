use bevy::input::mouse::AccumulatedMouseMotion;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        set_movement.run_if(input_just_pressed(KeyCode::KeyG)),
    );
    app.add_systems(Update, (movement, set_axis, draw_axis_gizmo));
    app.add_systems(
        Update,
        cancel_movement.run_if(input_just_pressed(KeyCode::Escape)),
    );
    app.add_systems(
        Update,
        finish_movement.run_if(input_just_pressed(MouseButton::Left)),
    );
}

#[derive(Component, Default)]
struct Moving {
    start_pos: Vec3,
    axis: Option<Axis>,
    local: bool,
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn get_direction(&self, local: bool, transform: &Transform) -> Vec3 {
        if local {
            match self {
                Self::X => Vec3::X,
                Self::Y => Vec3::Y,
                Self::Z => Vec3::Z,
            }
        } else {
            match self {
                Self::X => transform.local_x(),
                Self::Y => transform.local_y(),
                Self::Z => transform.local_z(),
            }
            .as_vec3()
        }
    }
}

fn set_movement(mut commands: Commands, focused: Res<FocusedEntities>, query: Query<&Transform>) {
    if focused.entities.is_empty() {
        debug!("No entity focused, so cannot move any!");
        return;
    }
    for entity in focused.entities.clone() {
        let Ok(transform) = query.get(entity) else {
            warn!("Focused entity: {entity} does not have a Transform component");
            continue;
        };
        commands.entity(entity).insert(Moving {
            start_pos: transform.translation,
            ..Default::default()
        });
    }
}

const SENSITIVITY: f32 = 0.01;
fn movement(
    motion: Res<AccumulatedMouseMotion>,
    query: Query<(&mut Transform, &Moving)>,
    camera: Single<&Transform, (With<EditorCamera>, Without<Moving>)>,
) {
    for (mut transform, moving) in query {
        if let Some(axis) = moving.axis {
            let dir = axis.get_direction(moving.local, &transform);
            let delta_x = match axis {
                Axis::Z => -motion.delta.x,
                _ => motion.delta.x,
            };
            let delta_y = match axis {
                Axis::Y => -motion.delta.y,
                _ => motion.delta.y,
            };
            transform.translation += dir * (delta_x + delta_y) * SENSITIVITY;
        } else {
            transform.translation += camera.right().as_vec3() * motion.delta.x * SENSITIVITY
                + camera.down().as_vec3() * motion.delta.y * SENSITIVITY;
        }
    }
}

fn set_axis(input: Res<ButtonInput<KeyCode>>, query: Query<&mut Moving>) {
    for mut moving in query {
        if input.just_pressed(KeyCode::KeyX) {
            if moving.axis.as_ref().is_some_and(|a| matches!(a, Axis::X)) {
                moving.local = !moving.local;
            } else {
                moving.axis = Some(Axis::X);
                moving.local = false;
            }
        }
        // This is German TD, fuck QWERTY
        if input.just_pressed(KeyCode::KeyZ) {
            if moving.axis.as_ref().is_some_and(|a| matches!(a, Axis::Y)) {
                moving.local = !moving.local;
            } else {
                moving.axis = Some(Axis::Y);
                moving.local = false;
            }
        }
        // This is German TD, fuck QWERTY
        if input.just_pressed(KeyCode::KeyY) {
            if moving.axis.as_ref().is_some_and(|a| matches!(a, Axis::Z)) {
                moving.local = !moving.local;
            } else {
                moving.axis = Some(Axis::Z);
                moving.local = false;
            }
        }
    }
}

fn draw_axis_gizmo(mut gizmos: Gizmos, query: Query<(&Transform, &Moving)>) {
    for (transform, moving) in query {
        if let Some(axis) = moving.axis {
            let dir = axis.get_direction(moving.local, transform);
            let color = match axis {
                Axis::X => Color::srgb(1., 0., 0.),
                Axis::Y => Color::srgb(0., 0., 1.),
                Axis::Z => Color::srgb(0., 1., 0.),
            };
            gizmos.line(
                transform.translation - dir * 100.,
                transform.translation + dir * 100.,
                color,
            );
        }
    }
}

fn cancel_movement(mut commands: Commands, query: Query<(Entity, &mut Transform, &Moving)>) {
    for (entity, mut transform, moving) in query {
        transform.translation = moving.start_pos;
        commands.entity(entity).remove::<Moving>();
    }
}

fn finish_movement(mut commands: Commands, query: Query<Entity, With<Moving>>) {
    for entity in query {
        commands.entity(entity).remove::<Moving>();
    }
}
