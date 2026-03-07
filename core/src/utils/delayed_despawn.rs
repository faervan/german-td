use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (delayed_despawn, delayed_children_despawn, despawn_after),
    );
}

#[derive(Component)]
/// Despawn an [`Entity`] in the next frame. Useful for despawning in a [`Component::on_add`] hook,
/// as immediately despawning there may trigger a panic.
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
/// Despawn the children of an [`Entity`] in the next frame. Useful for despawning in a
/// [`Component::on_add`] hook, as immediately despawning there may trigger a panic.
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct DespawnAfter(Timer);

impl DespawnAfter {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

fn despawn_after(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &mut DespawnAfter)>,
) {
    for (entity, mut despawn_after) in query {
        despawn_after.0.tick(time.delta());
        if despawn_after.0.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
