use bevy::{
    pbr::decal::{ForwardDecal, ForwardDecalMaterial, ForwardDecalMaterialExt},
    render::render_resource::AsBindGroup,
};

use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_plugins(MaterialPlugin::<ForwardDecalMaterial<TowerPlotMaterial>>::default());
        app.add_plugins(MaterialPlugin::<TowerRingMaterial>::default());
        app.add_plugins(MaterialPlugin::<TowerRingActionMaterial>::default());

        app.add_message::<SpawnTowerPlacement>();

        app.add_systems(
            Update,
            spawn_placements.run_if(on_message::<SpawnTowerPlacement>.and(in_state(game_state))),
        );
        app.add_systems(Update, blend_hover.run_if(in_state(game_state)));
    }
}

#[derive(Message, Debug)]
pub struct SpawnTowerPlacement {
    pub position: Vec3,
}

#[derive(Component)]
struct UnassignedTowerPlot;

fn spawn_placements(
    mut events: MessageReader<SpawnTowerPlacement>,
    mut commands: Commands,
    mut decal_standard_materials: ResMut<Assets<ForwardDecalMaterial<TowerPlotMaterial>>>,
) {
    for spawn in events.read() {
        commands
            .spawn((
                Name::new("Tower plot"),
                UnassignedTowerPlot,
                ForwardDecal,
                MeshMaterial3d(decal_standard_materials.add(ForwardDecalMaterial {
                    base: TowerPlotMaterial::default(),
                    extension: ForwardDecalMaterialExt {
                        depth_fade_factor: 1.,
                    },
                })),
                Transform::from_translation(spawn.position).with_scale(Vec3::splat(10.)),
                // Use physics picking, as there is no mesh picking for decals
                PhysicsPickable,
                RigidBody::Static,
                Collider::cylinder(0.5, 0.1),
            ))
            .observe(plot_change_hover_state::<Pointer<Over>, true>)
            .observe(plot_change_hover_state::<Pointer<Out>, false>)
            .observe(
                |event: On<Pointer<Click>>,
                 mut focused_ui: ResMut<FocusedUi>,
                 query: Query<&Transform>,
                 mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut ring_materials: ResMut<Assets<TowerRingMaterial>>,
                 tower_lib: TowerLibrary,
                 tower_defs: Res<Assets<TowerDefinition>>| {
                    if let Ok(transform) = query.get(event.entity) {
                        let ring_id = commands
                            .spawn((
                                Name::new("Tower selection thing"),
                                Transform::from_translation(transform.translation + Vec3::Y * 6.),
                                Billboarded,
                                Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(7.)))),
                                MeshMaterial3d(ring_materials.add(TowerRingMaterial {})),
                                NotShadowCaster,
                                NotShadowReceiver,
                            ))
                            .id();
                        focused_ui.register_focus(ring_id);

                        let mut actions = vec![];
                        for (handle, def) in tower_lib.entries.values().filter_map(|handle| {
                            tower_defs.get(handle).map(|def| (handle.clone(), def))
                        }) {
                            let mut entity_cmds = commands.spawn(ChildOf(ring_id));

                            let position = transform.translation;
                            let plot_id = event.entity;

                            let on_click = move |_event: On<Pointer<Click>>,
                                 mut commands: Commands,
                                 mut tower_spawner: MessageWriter<SpawnTower>| {
                                    commands.entity(plot_id).despawn();
                                    commands.entity(ring_id).despawn();
                                    tower_spawner.write(SpawnTower {
                                        position,
                                        definition: handle.clone(),
                                    });
                                };

                            let entity_id = entity_cmds.id();
                            entity_cmds.insert(Observer::new(on_click).with_entity(entity_id));
                            actions.push((entity_id, def.icon.clone()));
                        }

                        commands
                            .entity(ring_id)
                            .insert(TowerPlacementRing { actions })
                            .observe(
                                |event: On<Pointer<Click>>, mut focused_ui: ResMut<FocusedUi>| {
                                    focused_ui.register_click(event.entity);
                                },
                            );
                    }
                },
            );
    }
}

fn plot_change_hover_state<EVENT: EntityEvent, const SET_HOVERED: bool>(
    event: On<EVENT>,
    query: Query<&MeshMaterial3d<ForwardDecalMaterial<TowerPlotMaterial>>>,
    mut materials: ResMut<Assets<ForwardDecalMaterial<TowerPlotMaterial>>>,
) {
    if let Ok(handle) = query.get(event.event_target())
        && let Some(material) = materials.get_mut(&handle.0)
    {
        material.base.hovered = SET_HOVERED;
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
struct TowerPlotMaterial {
    hovered: bool,
    #[uniform(0)]
    /// Always between 0 and 1, 0 is not hovered, 1 means it is hovered
    blend_hover: f32,
}

impl Material for TowerPlotMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/tower_plot.wgsl".into()
    }
}

#[derive(Component)]
#[component(on_add)]
/// TODO! Maybe rename this, as it is used for the upgrade ring as well
struct TowerPlacementRing {
    actions: Vec<(Entity, Handle<Image>)>,
}

impl TowerPlacementRing {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mesh = meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(2.5)));

        let this = &mut world.get_mut::<Self>(hook.entity).unwrap();
        let action_len = this.actions.len() as f32;

        for (index, (entity, image)) in std::mem::take(&mut this.actions).into_iter().enumerate() {
            let mut materials = world.resource_mut::<Assets<TowerRingActionMaterial>>();
            let material = materials.add(TowerRingActionMaterial {
                icon: image,
                ..Default::default()
            });

            let angle = TAU / action_len * index as f32;
            let offset = (Vec3::Y * 6.5 + Vec3::Z * 0.01).rotate_z(angle);

            world
                .commands()
                .entity(entity)
                .insert((
                    Name::new("Place tower"),
                    Transform::from_translation(offset),
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material),
                    NotShadowCaster,
                ))
                .observe(action_icon_change_hover_state::<Pointer<Over>, true>)
                .observe(action_icon_change_hover_state::<Pointer<Out>, false>);
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
struct TowerRingMaterial {}

impl Material for TowerRingMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/tower_ring.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
struct TowerRingActionMaterial {
    hovered: bool,
    #[texture(0)]
    #[sampler(1)]
    icon: Handle<Image>,
    #[uniform(2)]
    /// Always between 0 and 1, 0 is not hovered, 1 means it is hovered
    blend_hover: f32,
}

impl Material for TowerRingActionMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/tower_ring_action.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

fn action_icon_change_hover_state<EVENT: EntityEvent, const SET_HOVERED: bool>(
    event: On<EVENT>,
    mut commands: Commands,
    query: Query<(&Transform, &MeshMaterial3d<TowerRingActionMaterial>)>,
    mut materials: ResMut<Assets<TowerRingActionMaterial>>,
) {
    if let Ok((transform, handle)) = query.get(event.event_target())
        && let Some(material) = materials.get_mut(&handle.0)
    {
        material.hovered = SET_HOVERED;
        let scale = match SET_HOVERED {
            true => Vec3::splat(1.1),
            false => Vec3::splat(1.),
        };
        commands.entity(event.event_target()).animate_towards(
            transform.with_scale(scale),
            Duration::from_secs_f32(1. / ACTION_ICON_BLEND_SPEED),
        );
    }
}

const PLOT_BLEND_SPEED: f32 = 15.;
const ACTION_ICON_BLEND_SPEED: f32 = 15.;
fn blend_hover(
    time: Res<Time>,
    mut plot_materials: ResMut<Assets<ForwardDecalMaterial<TowerPlotMaterial>>>,
    mut icon_materials: ResMut<Assets<TowerRingActionMaterial>>,
) {
    for (_, material) in plot_materials.iter_mut() {
        if material.base.hovered && material.base.blend_hover != 1.
            || !material.base.hovered && material.base.blend_hover != 0.
        {
            let sign = match material.base.hovered {
                true => 1.,
                false => -1.,
            };
            material.base.blend_hover += sign * time.delta_secs() * PLOT_BLEND_SPEED;
            material.base.blend_hover = material.base.blend_hover.clamp(0., 1.);
        }
    }
    for (_, material) in icon_materials.iter_mut() {
        if material.hovered && material.blend_hover != 1.
            || !material.hovered && material.blend_hover != 0.
        {
            let sign = match material.hovered {
                true => 1.,
                false => -1.,
            };
            material.blend_hover += sign * time.delta_secs() * ACTION_ICON_BLEND_SPEED;
            material.blend_hover = material.blend_hover.clamp(0., 1.);
        }
    }
}
