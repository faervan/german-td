use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    scene::SceneInstanceReady,
};

use crate::prelude::*;

pub fn on_ready_insert_animation_target(
    event: On<SceneInstanceReady>,
    mut commands: Commands,
    query: Query<(Option<&Children>, Has<AnimationPlayer>)>,
) {
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
    commands.entity(event.observer()).despawn();
}

pub fn on_ready_insert_mesh_picking(
    event: On<SceneInstanceReady>,
    mut commands: Commands,
    query: Query<(Option<&Children>, Has<Mesh3d>)>,
) {
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
    commands.entity(event.observer()).despawn();
}

pub fn on_ready_extend_material<B: Material + Clone, E: MaterialExtension + Default>(
    event: On<SceneInstanceReady>,
    mut commands: Commands,
    base_materials: Res<Assets<B>>,
    mut materials: ResMut<Assets<ExtendedMaterial<B, E>>>,
    query: Query<(Option<&Children>, Option<&MeshMaterial3d<B>>)>,
) {
    let root_entity = event.entity;
    let mut current = VecDeque::from([root_entity]);

    // Search for all [`MeshMaterial3d`] and replace them
    while let Some(entity) = current.pop_front() {
        if let Ok((children_maybe, material_maybe)) = query.get(entity) {
            if let Some(material) = material_maybe
                && let Some(base_material) = base_materials.get(material)
            {
                commands
                    .entity(entity)
                    .remove::<MeshMaterial3d<B>>()
                    .insert(MeshMaterial3d(materials.add(ExtendedMaterial {
                        base: base_material.clone(),
                        extension: E::default(),
                    })));
            }
            if let Some(children) = children_maybe {
                current.extend(children);
            }
        }
    }

    // Remove this observer
    commands.entity(event.observer()).despawn();
}
