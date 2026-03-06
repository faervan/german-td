use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_message::<SpawnProjectile>();

        app.add_systems(
            Update,
            (
                spawn_projectile.run_if(on_message::<SpawnProjectile>),
                move_projectile,
            )
                .run_if(in_state(game_state)),
        );
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Projectile {
    target: Entity,
}

#[derive(Message, Debug)]
pub struct SpawnProjectile {
    pub position: Vec3,
    pub target: Entity,
    pub definition: Handle<ProjectileDefinition>,
}

fn spawn_projectile(
    mut events: MessageReader<SpawnProjectile>,
    mut commands: Commands,
    definitions: Res<Assets<ProjectileDefinition>>,
) {
    for spawn in events.read() {
        let def = definitions.get(&spawn.definition).unwrap();

        commands.spawn((
            Name::new(format!("Projectile: {}", def.name)),
            Transform::from_translation(spawn.position),
            SceneRoot(def.scene.clone()),
            Projectile {
                target: spawn.target,
            },
            // Physics
            RigidBody::Kinematic,
            Collider::cylinder(0.3, 1.5),
            GravityScale(0.),
        ));
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
