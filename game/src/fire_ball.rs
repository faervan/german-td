use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.load_assets::<FireBallTextures>();
    app.add_systems(Update, rotate_trail);
}

#[derive(Resource, Asset, TypePath)]
struct FireBallTextures {
    trail: Handle<Image>,
}

impl FromWorld for FireBallTextures {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            trail: asset_server.load("textures/FireBallTrail.png"),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
struct FireBallCore;

#[derive(Component, Reflect)]
#[reflect(Component)]
#[component(on_add)]
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
}

impl FireBallTrail {
    fn on_add(mut world: DeferredWorld, hook: HookContext) {
        let trail = world.resource::<FireBallTextures>().trail.clone();
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
            // 0ms is roughly the frame duration on my system
            transform.rotate_y(0.003 * TRAIL_ROTATION_SPEED);
        } else {
            transform.rotate_y(time.delta_secs() * TRAIL_ROTATION_SPEED);
        }
        #[cfg(not(feature = "dev"))]
        transform.rotate_y(time.delta_secs());
    }
}
