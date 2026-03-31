use bevy::{
    asset::RenderAssetUsages,
    camera::RenderTarget,
    pbr::decal::{ForwardDecal, ForwardDecalMaterial, ForwardDecalMaterialExt},
    render::render_resource::{
        AsBindGroup, Extent3d, TextureDimension, TextureFormat, TextureUsages,
    },
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

        #[cfg(not(feature = "editor"))]
        app.add_systems(Update, update_gold_cost_ui.run_if(in_state(game_state)));
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
                 query: Query<&Transform>,
                 mut commands: Commands,
                 tower_lib: TowerLibrary,
                 tower_defs: Res<Assets<TowerDefinition>>| {
                    if let Ok(transform) = query.get(event.entity) {
                        let mut towers = tower_lib
                            .entries
                            .values()
                            .filter_map(|handle| {
                                tower_defs
                                    .get(handle)
                                    .map(|def| (handle.clone(), def.cost, def.icon.clone()))
                            })
                            .collect::<Vec<_>>();
                        towers.sort_by(|(_, x, _), (_, y, _)| y.cmp(x));
                        let position = transform.translation;
                        let plot_id = event.entity;
                        commands.run_system_cached_with(
                            spawn_tower_ring,
                            (
                                Box::new(
                                    move |action_spawner, ring_id, actions_top, _actions_bottom| {
                                        for (handle, cost, icon) in towers.clone().into_iter() {
                                            let mut entity_cmds = action_spawner.spawn_empty();

                                            let on_click =
                                                move |_event: On<Pointer<Click>>,
                                                      mut commands: Commands,
                                                      mut tower_spawner: MessageWriter<
                                                    SpawnTower,
                                                >| {
                                                    commands.entity(plot_id).despawn();
                                                    commands.entity(ring_id).despawn();
                                                    tower_spawner.write(SpawnTower {
                                                        position,
                                                        definition: handle.clone(),
                                                    });
                                                };

                                            let entity_id = entity_cmds.id();
                                            entity_cmds.insert((
                                                Observer::new(on_click).with_entity(entity_id),
                                                TowerRingAction { cost },
                                            ));
                                            actions_top.push((entity_id, icon));
                                        }
                                    },
                                ),
                                transform.translation,
                            ),
                        );
                    }
                },
            );
    }
}

pub(super) fn spawn_tower_ring(
    (add_actions, position): (
        In<
            Box<
                dyn Fn(
                        &mut ChildSpawnerCommands,
                        Entity,
                        &mut Vec<(Entity, Handle<Image>)>,
                        &mut Vec<(Entity, Handle<Image>)>,
                    ) + Send,
            >,
        >,
        In<Vec3>,
    ),
    mut commands: Commands,
    mut focused_ui: ResMut<FocusedUi>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ring_materials: ResMut<Assets<TowerRingMaterial>>,
) {
    let mut ring_cmds = commands.spawn((
        Name::new("Tower selection ring"),
        // TODO! figure out a better way to ensure the ring is in front and can be accessible
        // (especially is not blocked by the tower it was activated for)
        Transform::from_translation(*position + Vec3::Y * 10. + Vec3::Z * 10.),
        Billboarded,
        Pickable::default(),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(7.)))),
        MeshMaterial3d(ring_materials.add(TowerRingMaterial {})),
        NotShadowCaster,
        NotShadowReceiver,
    ));
    let ring_id = ring_cmds.id();
    focused_ui.register_focus(ring_id);

    let mut actions_top = vec![];
    let mut actions_bottom = vec![];
    ring_cmds.with_children(|action_spawner| {
        add_actions(
            action_spawner,
            ring_id,
            &mut actions_top,
            &mut actions_bottom,
        );
    });

    ring_cmds
        .insert(TowerActionRing::new(actions_top, actions_bottom))
        .observe(
            |event: On<Pointer<Click>>, mut focused_ui: ResMut<FocusedUi>| {
                focused_ui.register_click(event.entity);
            },
        );
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

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
struct TowerActionRing {
    actions_top: Vec<(Entity, Handle<Image>)>,
    actions_bottom: Vec<(Entity, Handle<Image>)>,
    /// The amount of gold needed to perform the currently hovered action
    hovered_action_cost: Option<usize>,
    /// The [`Entity`] holding the [`Text`] node to display the gold cost
    cost_text_entity: Entity,
}

impl TowerActionRing {
    const ACTION_ANGLE_GAP: f32 = TAU / 6.;

    fn new(
        actions_top: Vec<(Entity, Handle<Image>)>,
        actions_bottom: Vec<(Entity, Handle<Image>)>,
    ) -> Self {
        Self {
            actions_top,
            actions_bottom,
            hovered_action_cost: None,
            cost_text_entity: Entity::PLACEHOLDER,
        }
    }

    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let mesh = meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(2.5)));

        world
            .commands()
            .run_system_cached_with(setup_gold_cost_ui, hook.entity);

        let this = &mut world.get_mut::<Self>(hook.entity).unwrap();
        let actions_top_len = this.actions_top.len() as f32;
        let actions_bottom_len = this.actions_bottom.len() as f32;

        for (index, (entity, image, is_at_top)) in std::mem::take(&mut this.actions_top)
            .into_iter()
            .map(|(a, b)| (a, b, true))
            .enumerate()
            .chain(
                std::mem::take(&mut this.actions_bottom)
                    .into_iter()
                    .map(|(a, b)| (a, b, false))
                    .enumerate(),
            )
        {
            let mut materials = world.resource_mut::<Assets<TowerRingActionMaterial>>();
            let material = materials.add(TowerRingActionMaterial {
                icon: image,
                ..Default::default()
            });

            let angle = match is_at_top {
                true => Self::ACTION_ANGLE_GAP * (0.5 + index as f32 - actions_top_len / 2.),
                false => {
                    PI + Self::ACTION_ANGLE_GAP * (0.5 + index as f32 - actions_bottom_len / 2.)
                }
            };
            let offset = (Vec3::Y * 6.5 + Vec3::Z * 0.01).rotate_z(angle);

            world
                .commands()
                .entity(entity)
                .insert((
                    Name::new("Place tower"),
                    Transform::from_translation(offset),
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material),
                    Pickable::default(),
                    NotShadowCaster,
                ))
                .observe(action_icon_change_hover_state::<Pointer<Over>, true>)
                .observe(action_icon_change_hover_state::<Pointer<Out>, false>);
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
pub(super) struct TowerRingMaterial {}

impl Material for TowerRingMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/tower_ring.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
/// This naming is a bit weird since there also is [`TowerActionRing`], but the difference should
/// still be understandable I guess :)
pub(super) struct TowerRingAction {
    /// Gold required for this action
    pub(super) cost: usize,
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
    query: Query<(
        &Transform,
        &MeshMaterial3d<TowerRingActionMaterial>,
        &ChildOf,
        &TowerRingAction,
    )>,
    mut ring_query: Query<&mut TowerActionRing>,
    mut materials: ResMut<Assets<TowerRingActionMaterial>>,
) {
    if let Ok((transform, handle, parent, action)) = query.get(event.event_target())
        && let Some(material) = materials.get_mut(&handle.0)
    {
        if let Ok(mut ring) = ring_query.get_mut(parent.0) {
            ring.hovered_action_cost = SET_HOVERED.then_some(action.cost);
        }
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

/// see <https://bevy.org/examples/ui-user-interface/render-ui-to-texture/>
fn setup_gold_cost_ui(
    ring_id: In<Entity>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<&mut TowerActionRing>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);

    let texture_camera = commands
        .spawn((
            Camera2d,
            Camera {
                // render before the "main pass" camera
                order: -1,
                clear_color: ClearColorConfig::Custom(Color::default().with_alpha(0.)),
                ..default()
            },
            RenderTarget::Image(image_handle.clone().into()),
        ))
        .id();

    let root = commands
        .spawn((
            Node {
                // Cover the whole image
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            UiTargetCamera(texture_camera),
        ))
        .id();

    let text_id = commands
        .spawn((
            Text::new(""),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor::WHITE,
            ChildOf(root),
        ))
        .id();

    let mesh = meshes.add(Plane3d::new(Vec3::Z, Vec2::splat(10.)));
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    commands.spawn((
        Name::new("Gold cost texture"),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0., 0., 0.1),
        Pickable::IGNORE,
        ChildOf(*ring_id),
    ));

    if let Ok(mut ring) = query.get_mut(*ring_id) {
        ring.cost_text_entity = text_id;
    }
}

#[cfg(not(feature = "editor"))]
fn update_gold_cost_ui(
    gold: Res<Gold>,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
    query: Query<&TowerActionRing>,
) {
    for ring in query {
        if let Ok((mut text, mut color)) = text_query.get_mut(ring.cost_text_entity) {
            text.0 = match ring.hovered_action_cost {
                Some(cost) => {
                    color.0 = match gold.0 >= cost {
                        true => Color::srgb(1., 1., 0.),
                        false => Color::srgb(1., 0., 0.),
                    };
                    format!("Gold: {cost}")
                }
                None => String::new(),
            };
        }
    }
}
