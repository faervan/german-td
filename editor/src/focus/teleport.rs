use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(click);
}

fn click(
    event: On<Pointer<Click>>,
    meshes: Query<(), With<Mesh3d>>,
    input: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    focused: Res<FocusedEntities>,
    mut query: Query<&mut Transform, With<FocusableEntity>>,
) {
    if meshes.contains(event.entity)
        && input.pressed(KeyCode::ControlLeft)
        && !mouse.pressed(MouseButton::Left)
        && !mouse.pressed(MouseButton::Middle)
        && !mouse.pressed(MouseButton::Back)
        && !mouse.pressed(MouseButton::Forward)
    {
        let Some(position) = event.hit.position else {
            return;
        };
        for entity in &focused.entities {
            let Ok(mut transform) = query.get_mut(*entity) else {
                continue;
            };
            transform.translation = position;
        }
    }
}
