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
    damage: f32,
}

#[derive(Message, Debug)]
pub struct SpawnProjectile {
    pub position: Vec3,
    pub target: Entity,
    pub definition: Handle<ProjectileDefinition>,
    /// A multiplier to the projectiles base damage depending on the tower that shot it
    pub damage_factor: f32,
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
                damage: def.damage * spawn.damage_factor,
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
    projectile_transforms: Query<(Entity, &mut Transform, &mut LinearVelocity, &Projectile)>,
    mut targets: Query<(Entity, &Transform, &mut Health), Without<Projectile>>,
    time: Res<Time>,
    audio_handles: Res<GameSoundHandles>,
) {
    for (entity, mut projectile_transform, mut projectile_velocity, projectile) in
        projectile_transforms
    {
        let mut despawn = false;

        if let Ok((target_entity, target_transform, mut target_health)) =
            targets.get_mut(projectile.target)
        {
            let direction = target_transform.translation - projectile_transform.translation;

            projectile_transform.look_at(target_transform.translation, Vec3::Y);
            projectile_velocity.0 = direction.normalize() * 30.0;

            if direction.length() < 1.0 {
                despawn = true;
                target_health.0 -= projectile.damage;
                if target_health.0.is_sign_negative() {
                    commands.entity(target_entity).despawn();
                    commands.spawn(sound_effect(audio_handles.enemy_death_from_time(&time)));
                }
            }
        } else {
            despawn = true;
        }

        if despawn {
            commands.entity(entity).despawn();
        }
    }
}
