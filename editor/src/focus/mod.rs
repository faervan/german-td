use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    render::render_resource::AsBindGroup,
};

use crate::prelude::*;

mod movement;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(movement::plugin);

    app.init_resource::<FocusedEntities>();

    app.add_plugins(MeshPickingPlugin);
    app.add_plugins(MaterialPlugin::<
        ExtendedMaterial<StandardMaterial, FocusMaterial>,
    >::default());

    app.add_systems(Update, (add_pickable, draw_axes));
    app.add_systems(
        Update,
        clear_focused
            .run_if(input_pressed(KeyCode::ShiftLeft).and(input_just_pressed(KeyCode::Escape))),
    );
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct FocusedEntities {
    pub entities: Vec<Entity>,
}

fn clear_focused(mut commands: Commands, mut focused: ResMut<FocusedEntities>) {
    for entity in focused.entities.drain(..) {
        commands
            .entity(entity)
            .trigger(|target| EntitySelectChange {
                target,
                selected: false,
            });
    }
}

#[derive(Component)]
pub struct FocusableEntity;

#[derive(EntityEvent, Clone)]
struct EntityHoverChange {
    #[event_target]
    target: Entity,
    hovered: bool,
}

#[derive(EntityEvent, Clone)]
pub struct EntitySelectChange {
    #[event_target]
    pub target: Entity,
    pub selected: bool,
}

#[derive(Component)]
struct MaterialHandles(Vec<Handle<ExtendedMaterial<StandardMaterial, FocusMaterial>>>);

fn trigger_root_event<E: EntityEvent, TRIGGER: EntityEvent + Clone>(
    trigger: TRIGGER,
) -> impl Fn(On<E>, Commands)
where
    for<'a> <TRIGGER as bevy::prelude::Event>::Trigger<'a>: std::default::Default,
{
    move |_event, mut commands| {
        commands.trigger(trigger.clone());
    }
}

fn edit_material<E: EntityEvent>(
    f: impl Fn(&E, &mut f32),
) -> impl Fn(
    On<E>,
    Query<&MaterialHandles>,
    ResMut<Assets<ExtendedMaterial<StandardMaterial, FocusMaterial>>>,
) {
    move |event: On<E>,
          query: Query<&MaterialHandles>,
          mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FocusMaterial>>>| {
        if let Ok(handles) = query.get(event.event_target()) {
            for handle in &handles.0 {
                if let Some(material) = materials.get_mut(handle) {
                    f(&event, &mut material.extension.hightlight);
                }
            }
        }
    }
}

fn add_pickable(
    mut commands: Commands,
    added: Query<
        (Entity, Option<&Children>),
        Or<(Added<Enemy>, Added<Tower>, Added<EditorCursor>)>,
    >,
    query: Query<(Option<&Children>, Option<&MeshMaterial3d<StandardMaterial>>), With<ChildOf>>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FocusMaterial>>>,
) {
    for (root, children_maybe) in added {
        commands
            .entity(root)
            .insert(FocusableEntity)
            .observe(
                |event: On<EntitySelectChange>,
                 mut commands: Commands,
                 input: Res<ButtonInput<KeyCode>>,
                 mut focused: ResMut<FocusedEntities>| {
                    if event.selected {
                        if input.pressed(KeyCode::ShiftLeft) {
                            focused.entities.push(event.event_target());
                        } else {
                            for entity in std::mem::take(&mut focused.entities) {
                                if entity == event.event_target() {
                                    continue;
                                }
                                commands
                                    .entity(entity)
                                    .trigger(|target| EntitySelectChange {
                                        target,
                                        selected: false,
                                    });
                            }
                            focused.entities = vec![event.event_target()];
                        }
                    }
                },
            )
            .observe(edit_material::<EntitySelectChange>(
                |e, hightlight| match e.selected {
                    true => *hightlight = 2.,
                    false => *hightlight = 0.,
                },
            ))
            .observe(edit_material::<EntityHoverChange>(|e, highlight| {
                if *highlight != 2. {
                    *highlight = match e.hovered {
                        true => 1.,
                        false => 0.,
                    };
                }
            }));
        let mut material_handles = vec![];
        let Some(children) = children_maybe else {
            commands
                .entity(root)
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
                ));
            continue;
        };
        let mut current = children.to_vec();
        while !current.is_empty() {
            for entity in std::mem::take(&mut current) {
                if let Ok((children_maybe, material_maybe)) = query.get(entity) {
                    if let Some(children) = children_maybe {
                        current.extend(children);
                    }
                    if let Some(base_material) =
                        material_maybe.and_then(|handle| standard_materials.get(handle).cloned())
                    {
                        let extended_material = materials.add(ExtendedMaterial {
                            base: base_material,
                            extension: FocusMaterial::default(),
                        });
                        material_handles.push(extended_material.clone());
                        commands
                            .entity(entity)
                            .remove::<MeshMaterial3d<StandardMaterial>>()
                            .insert(MeshMaterial3d(extended_material))
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
                            ));
                    }
                }
            }
        }
        commands
            .entity(root)
            .insert(MaterialHandles(material_handles));
    }
}

fn draw_axes(mut gizmos: Gizmos, query: Query<&Transform, With<FocusableEntity>>) {
    for transform in query {
        gizmos.axes(*transform, 3.);
    }
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
#[reflect(Asset)]
struct FocusMaterial {
    #[uniform(100)]
    /// 0 => default
    /// 1 => hovered
    /// 2 => selected
    hightlight: f32,
}

impl MaterialExtension for FocusMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/focus.wgsl".into()
    }

    fn deferred_fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/focus.wgsl".into()
    }
}
