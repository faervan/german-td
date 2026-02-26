use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_systems(Update, (move_projectile,).run_if(in_state(game_state)));
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Projectile {
    target: Entity,
}

impl Projectile {
    pub fn new(target: Entity) -> Self {
        Self { target }
    }
}

// Moves the projectile to the target
// TODO: split out despawn to get rid of commands
fn move_projectile(
    mut commands: Commands,
    mut projectile_transforms: Query<(Entity, &mut Transform, &mut LinearVelocity, &Projectile)>,
    other_transforms: Query<&Transform, Without<Projectile>>,
) {
    for (entity, mut projectile_transform, mut projectile_velocity, projectile) in
        &mut projectile_transforms
    {
        let mut despawn = false;

        if let Ok(target_transform) = other_transforms.get(projectile.target) {
            let direction = target_transform.translation - projectile_transform.translation;

            projectile_transform.look_at(target_transform.translation, Vec3::Y);
            projectile_velocity.0 = direction.normalize() * 30.0;

            if direction.length() < 1.0 {
                despawn = true;
            }
        } else {
            despawn = true;
        }

        if despawn {
            commands.entity(entity).despawn();
        }
    }
}
