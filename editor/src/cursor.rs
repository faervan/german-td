use bevy::picking::backend::{HitData, PointerHits, ray::RayMap};

use crate::{focus::EntitySelectChange, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(State::Editor), spawn_cursor);

    app.add_systems(Update, cursor_picking.run_if(in_state(State::Editor)));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct EditorCursor;

fn cursor_gizmo(selected: bool) -> GizmoAsset {
    let mut gizmo = GizmoAsset::new();
    let color = match selected {
        true => Color::srgb(0., 0., 1.),
        false => Color::srgb(1., 1., 0.),
    };
    gizmo.sphere(Isometry3d::IDENTITY, 1., color);
    gizmo
}

fn spawn_cursor(mut commands: Commands, mut gizmos: ResMut<Assets<GizmoAsset>>) {
    let gizmo = cursor_gizmo(false);

    commands
        .spawn((
            Name::new("3dCursor"),
            EditorCursor,
            FocusableEntity,
            Transform::default(),
            Gizmo {
                handle: gizmos.add(gizmo),
                line_config: GizmoLineConfig {
                    width: 5.,
                    ..Default::default()
                },
                depth_bias: 0.,
            },
        ))
        .observe(
            |event: On<EntitySelectChange>,
             gizmo: Single<&Gizmo, With<EditorCursor>>,
             mut gizmos: ResMut<Assets<GizmoAsset>>| {
                let Some(gizmo) = gizmos.get_mut(&gizmo.handle) else {
                    return;
                };
                *gizmo = cursor_gizmo(event.selected);
            },
        );
}

fn cursor_picking(
    ray_map: Res<RayMap>,
    mut output_messages: MessageWriter<PointerHits>,
    cursor: Single<(Entity, &Transform), With<EditorCursor>>,
) {
    for (&ray_id, &ray) in ray_map.iter() {
        let distance = (cursor.1.translation - ray.origin)
            .cross(*ray.direction)
            .length();
        if distance < 1. {
            output_messages.write(PointerHits {
                pointer: ray_id.pointer,
                picks: vec![(
                    cursor.0,
                    HitData {
                        camera: ray_id.camera,
                        depth: (cursor.1.translation - ray.origin).length(),
                        position: Some(cursor.1.translation),
                        normal: Some((ray.origin - cursor.1.translation).normalize_or_zero()),
                    },
                )],
                order: 0.,
            });
        }
    }
}
