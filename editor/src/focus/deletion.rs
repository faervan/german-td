use crate::{focus::movement::Moving, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn_selected.run_if(input_just_pressed(KeyCode::KeyX)),
    );
}

/// TODO! only trigger when cursor is in 3d viewport (not over ui)
fn despawn_selected(
    mut commands: Commands,
    mut focused: ResMut<FocusedEntities>,
    query: Query<
        (),
        (
            With<FocusableEntity>,
            Without<Moving>,
            Without<EditorCursor>,
        ),
    >,
) {
    for entity in std::mem::take(&mut focused.entities) {
        if query.contains(entity) {
            commands.entity(entity).despawn();
        } else {
            focused.entities.push(entity);
        }
    }
}
