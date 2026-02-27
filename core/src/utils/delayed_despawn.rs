use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, (delayed_despawn, delayed_children_despawn));
}

#[derive(Component)]
pub struct DelayedDespawn;

fn delayed_despawn(
    mut commands: Commands,
    query: Query<Entity, Added<DelayedDespawn>>,
    mut despawn: Local<Vec<Entity>>,
) {
    for entity in despawn.drain(..) {
        commands.entity(entity).despawn();
    }

    for entity in query {
        despawn.push(entity);
    }
}

#[derive(Component)]
pub struct DelayedChildrenDespawn;

fn delayed_children_despawn(
    mut commands: Commands,
    query: Query<Entity, Added<DelayedChildrenDespawn>>,
    mut despawn: Local<Vec<Entity>>,
) {
    for entity in despawn.drain(..) {
        commands.entity(entity).despawn_children();
    }

    for entity in query {
        despawn.push(entity);
    }
}
