use crate::prelude::*;

pub(crate) fn plugin(app: &mut App) {
    app.add_systems(Update, (attack_tower_target, move_projectile));
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Tower {
    target: Option<Entity>,
    attack_timer: Timer,
}

impl Tower {
    // TODO: Remove target from here; just for testing
    pub fn new(target: Entity, attack_speed: f32) -> Self {
        /* Star finished */
        let mut attack_timer = Timer::from_seconds(attack_speed, TimerMode::Repeating);
        attack_timer.finish();

        Self {
            target: Some(target),
            attack_timer,
        }
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Projectile {
    target: Entity,
}

// Moves the projectile to the target
// TODO: Physics, rotation, split out despawn to get rid of commands
fn move_projectile(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_transforms: Query<(Entity, &mut Transform, &Projectile)>,
    other_transforms: Query<&Transform, Without<Projectile>>,
) {
    for (entity, mut projectile_transform, projectile) in &mut projectile_transforms {
        let mut despawn = false;

        if let Ok(target_transform) = other_transforms.get(projectile.target) {
            let direction = target_transform.translation - projectile_transform.translation;

            projectile_transform.translation += direction.normalize() * 25.0 * time.delta_secs();

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

// TODO: This can probably be moved into collision event hooks?
// Sets the target of the Tower Component
fn _search_tower_target() {}

// TODO: Move out spawning of projectile
fn attack_tower_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut towers: Query<(&mut Tower, &Transform)>,
) {
    for (mut tower, transform) in &mut towers {
        if tower.attack_timer.is_finished()
            && let Some(target) = tower.target
        {
            commands.spawn((
                Projectile { target },
                Mesh3d(meshes.add(Cuboid::from_length(3.0))),
                MeshMaterial3d(materials.add(Color::Srgba(Srgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.0,
                    alpha: 1.0,
                }))),
                *transform,
            ));
        }

        // tick timer
        tower.attack_timer.tick(time.delta());
    }
}
