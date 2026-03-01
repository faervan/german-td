use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        despawn_selected.run_if(input_just_pressed(KeyCode::KeyX)),
    );
}

/// TODO! only trigger when cursor is in 3d viewport
fn despawn_selected(mut commands: Commands, mut focused: ResMut<FocusedEntities>) {
    for entity in focused.entities.drain(..) {
        commands.entity(entity).despawn();
    }
}
