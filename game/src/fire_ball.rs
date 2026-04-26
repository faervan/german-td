use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.load_assets_with(FireBallAssets::new);
    app.add_systems(Update, (rotate_trail, drive_explosion));
}

#[derive(Asset, TypePath)]
struct FireBallAssetLoader {
    trail: Handle<Image>,
    impact: Handle<Gltf>,
    impact_inner_shell: Handle<Image>,
    impact_outer_shell: Handle<Image>,
}

impl FromWorld for FireBallAssetLoader {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            trail: asset_server.load("textures/FireBallTrail.png"),
            impact: asset_server.load("models/projectiles/FireBallImpact.glb"),
            impact_inner_shell: asset_server.load("textures/FireBallImpactInnerShell.png"),
            impact_outer_shell: asset_server.load("textures/FireBallImpactOuterShell.png"),
        }
    }
}

#[derive(Resource)]
struct FireBallAssets {
    trail: Handle<Image>,
    impact: Handle<Scene>,
    impact_inner_shell: Handle<Image>,
    impact_outer_shell: Handle<Image>,
}

impl FireBallAssets {
    fn new(asset: FireBallAssetLoader, world: &mut World) -> Self {
        let gltfs = world.resource::<Assets<Gltf>>();
        Self {
            trail: asset.trail,
            impact: gltfs
                .get(&asset.impact)
                .as_ref()
                .unwrap()
                .default_scene
                .clone()
                .unwrap(),
            impact_inner_shell: asset.impact_inner_shell,
            impact_outer_shell: asset.impact_outer_shell,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add, on_remove)]
#[require(NotShadowReceiver)]
struct FireBallCore;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[require(NotShadowCaster, NotShadowReceiver)]
struct FireBallTrail;

impl FireBallCore {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.),
            emissive: LinearRgba::new(1., 0.3, 0., 1.) * 200.,
            ..Default::default()
        });
        world
            .commands()
            .entity(hook.entity)
            .insert(MeshMaterial3d(material));
    }

    fn on_remove(mut world: DeferredWorld, hook: HookContext) {
        let pos = world
            .get::<GlobalTransform>(hook.entity)
            .unwrap()
            .translation();
        let scene = world.resource::<FireBallAssets>().impact.clone();
        world.commands().spawn((
            Name::new("FireBallImpact"),
            FireBallImpact { progress: 0. },
            SceneRoot(scene),
            Transform::from_translation(pos).with_scale(Vec3::ZERO),
        ));
    }
}

impl FireBallTrail {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let trail = world.resource::<FireBallAssets>().trail.clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(trail),
            emissive: LinearRgba::new(1., 0.3, 0., 1.) * 50.,
            alpha_mode: AlphaMode::Blend,
            ..Default::default()
        });
        world
            .commands()
            .entity(hook.entity)
            .insert(MeshMaterial3d(material));
    }
}

const TRAIL_ROTATION_SPEED: f32 = 5.;
fn rotate_trail(time: Res<Time<Virtual>>, trails: Query<&mut Transform, With<FireBallTrail>>) {
    for mut transform in trails {
        #[cfg(feature = "dev")]
        if time.is_paused() {
            // 3ms is roughly the frame duration on my system
            transform.rotate_y(0.003 * TRAIL_ROTATION_SPEED);
        } else {
            transform.rotate_y(time.delta_secs() * TRAIL_ROTATION_SPEED);
        }
        #[cfg(not(feature = "dev"))]
        transform.rotate_y(time.delta_secs());
    }
}

#[derive(Component)]
struct FireBallImpact {
    progress: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[require(NotShadowCaster, NotShadowReceiver)]
struct FireBallImpactCore;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[require(NotShadowCaster, NotShadowReceiver)]
struct FireBallImpactInnerShell {
    progress: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
#[require(NotShadowCaster, NotShadowReceiver)]
struct FireBallImpactOuterShell {
    progress: f32,
}

impl FireBallImpactCore {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(StandardMaterial {
            base_color: Color::srgb(1., 0., 0.),
            emissive: LinearRgba::new(1., 0.3, 0., 1.) * 200.,
            ..Default::default()
        });
        world
            .commands()
            .entity(hook.entity)
            .insert(MeshMaterial3d(material));
    }
}

impl FireBallImpactInnerShell {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let tex = world
            .resource::<FireBallAssets>()
            .impact_inner_shell
            .clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(tex),
            alpha_mode: AlphaMode::Mask(1.),
            ..Default::default()
        });
        world
            .commands()
            .entity(hook.entity)
            .insert(MeshMaterial3d(material));
    }
}

impl FireBallImpactOuterShell {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let tex = world
            .resource::<FireBallAssets>()
            .impact_outer_shell
            .clone();
        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let material = materials.add(StandardMaterial {
            base_color_texture: Some(tex),
            alpha_mode: AlphaMode::Mask(1.),
            ..Default::default()
        });
        world
            .commands()
            .entity(hook.entity)
            .insert(MeshMaterial3d(material));
    }
}

const EXPLOSION_SPEED: f32 = 2.;
fn drive_explosion(
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    impacts: Query<(Entity, &mut FireBallImpact, &mut Transform)>,
    inner_shells: Query<(
        &mut FireBallImpactInnerShell,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    outer_shells: Query<(
        &mut FireBallImpactOuterShell,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut commands: Commands,
) {
    for (entity, mut impact, mut transform) in impacts {
        impact.progress += time.delta_secs() * EXPLOSION_SPEED;
        // -(x+0.85)^{-4}+2
        transform.scale = Vec3::splat(-(impact.progress + 0.85).powf(-4.) + 2.);
        if impact.progress >= 1. {
            commands.entity(entity).despawn();
        }
    }
    for (mut shell, material) in inner_shells {
        shell.progress += time.delta_secs() * EXPLOSION_SPEED;
        if let Some(material) = materials.get_mut(&material.0) {
            material.alpha_mode = AlphaMode::Mask((shell.progress + 0.85).powf(-4.));
        }
    }
    for (mut shell, material) in outer_shells {
        shell.progress += time.delta_secs() * EXPLOSION_SPEED;
        if let Some(material) = materials.get_mut(&material.0) {
            material.alpha_mode = AlphaMode::Mask(1. - (shell.progress + 0.7).powf(-2.));
        }
    }
}
