use bevy::scene::SceneInstanceReady;

use crate::prelude::*;

pub fn on_ready_insert_animation_target(
    observer_entity: Entity,
    observed_entity: Entity,
) -> Observer {
    Observer::new(
        move |event: On<SceneInstanceReady>,
              mut commands: Commands,
              query: Query<(Option<&Children>, Has<AnimationPlayer>)>| {
            let mut current = vec![event.entity];

            // Search for an [`AnimationPlayer`] and assume the first one found is the armature we want
            'outer: loop {
                if current.is_empty() {
                    break;
                }
                for entity in std::mem::take(&mut current) {
                    if let Ok((children_maybe, has_player)) = query.get(entity) {
                        if has_player {
                            commands
                                .entity(event.entity)
                                .insert(GltfAnimationTarget(entity));
                            break 'outer;
                        }
                        if let Some(children) = children_maybe {
                            current.extend(children);
                        }
                    }
                }
            }

            // Remove this observer
            commands.entity(observer_entity).despawn();
        },
    )
    .with_entity(observed_entity)
}

pub fn on_ready_insert_mesh_picking(observer_entity: Entity, observed_entity: Entity) -> Observer {
    Observer::new(
        move |event: On<SceneInstanceReady>,
              mut commands: Commands,
              query: Query<(Option<&Children>, Has<Mesh3d>)>| {
            let root_entity = event.entity;
            let mut current = vec![root_entity];

            // Search for all [`Mesh3d`] and mark them as pickable
            loop {
                if current.is_empty() {
                    break;
                }
                for entity in std::mem::take(&mut current) {
                    if let Ok((children_maybe, has_mesh)) = query.get(entity) {
                        if has_mesh {
                            // Will be automatically propagated up to the root entity by bevy's ECS
                            commands.entity(entity).insert(Pickable::default());
                        }
                        if let Some(children) = children_maybe {
                            current.extend(children);
                        }
                    }
                }
            }

            // Remove this observer
            commands.entity(observer_entity).despawn();
        },
    )
    .with_entity(observed_entity)
}
