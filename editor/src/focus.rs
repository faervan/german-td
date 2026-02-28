use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<FocusedEntities>();

    app.add_plugins(MeshPickingPlugin);

    app.add_systems(Update, (add_pickable, draw_axes));

    app.world_mut().add_observer(print_click);
}

#[derive(Resource, Default)]
struct FocusedEntities {
    entities: Vec<Entity>,
}

#[derive(Component)]
struct FocusableEntity;

#[derive(EntityEvent, Clone, Copy)]
struct EntityHoverChange {
    #[event_target]
    target: Entity,
    hovered: bool,
}

#[derive(EntityEvent, Clone, Copy)]
struct EntitySelectChange {
    #[event_target]
    target: Entity,
    selected: bool,
}

fn trigger_root_event<E: EntityEvent, TRIGGER: EntityEvent + Copy>(
    trigger: TRIGGER,
) -> impl Fn(On<E>, Commands)
where
    for<'a> <TRIGGER as bevy::prelude::Event>::Trigger<'a>: std::default::Default,
{
    move |_event, mut commands| {
        commands.trigger(trigger);
    }
}

fn add_pickable(
    mut commands: Commands,
    added: Query<(Entity, &Children), Or<(Added<Enemy>, Added<Tower>)>>,
    query: Query<(Option<&Children>, Has<Mesh3d>), With<ChildOf>>,
) {
    for (root, children) in added {
        dbg!(root);
        commands
            .entity(root)
            .insert(FocusableEntity)
            .observe(move |event: On<EntityHoverChange>| match event.hovered {
                true => debug!("Entity {root} was hovered!"),
                false => debug!("Entity {root} was unhovered!"),
            })
            .observe(move |event: On<EntitySelectChange>| match event.selected {
                true => debug!("Entity {root} was selected!"),
                false => debug!("Entity {root} was unselected!"),
            });
        let mut current = children.to_vec();
        while !current.is_empty() {
            for entity in std::mem::take(&mut current) {
                if let Ok((children_maybe, has_mesh)) = query.get(entity) {
                    if let Some(children) = children_maybe {
                        current.extend(children);
                    }
                    if has_mesh {
                        commands
                            .entity(entity)
                            .observe(trigger_root_event::<Pointer<Over>, _>(EntityHoverChange {
                                target: root,
                                hovered: true,
                            }))
                            .observe(trigger_root_event::<Pointer<Out>, _>(EntityHoverChange {
                                target: root,
                                hovered: false,
                            }))
                            .observe(trigger_root_event::<Pointer<Press>, _>(
                                EntitySelectChange {
                                    target: root,
                                    selected: true,
                                },
                            ))
                            .observe(trigger_root_event::<Pointer<Release>, _>(
                                EntitySelectChange {
                                    target: root,
                                    selected: false,
                                },
                            ));
                    }
                }
            }
        }
    }
}

fn draw_axes(mut gizmos: Gizmos, query: Query<&Transform, With<FocusableEntity>>) {
    for transform in query {
        gizmos.axes(*transform, 3.);
    }
}

fn print_click(event: On<Pointer<Click>>) {
    debug!("Clicked on entity: {}", event.entity);
}
