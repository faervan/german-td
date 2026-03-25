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
    damage_type: DamageType,
}

#[derive(Message, Debug)]
pub struct SpawnProjectile {
    pub position: Vec3,
    pub target: Entity,
    pub definition: Handle<ProjectileDefinition>,
    /// A multiplier to the projectiles base damage depending on the tower that shot it
    pub damage_factor: f32,
    pub damage_type: DamageType,
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
                damage_type: spawn.damage_type,
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
    mut targets: Query<(Entity, &Transform, &mut Health, &Enemy), Without<Projectile>>,
    time: Res<Time>,
    audio_handles: Res<GameSoundHandles>,
    mut enemy_killed: MessageWriter<EnemyKilled>,
    spatial_query: SpatialQuery,
) {
    for (entity, mut projectile_transform, mut projectile_velocity, projectile) in
        projectile_transforms
    {
        let mut despawn = false;
        let mut entities_to_damage = vec![];

        if let Ok((target_entity, target_transform, _, _)) = targets.get(projectile.target) {
            let direction = target_transform.translation - projectile_transform.translation;

            projectile_transform.look_at(target_transform.translation, Vec3::Y);
            projectile_velocity.0 = direction.normalize() * 30.0;

            if direction.length() < 1.0 {
                despawn = true;

                match projectile.damage_type {
                    DamageType::Single => entities_to_damage.push(target_entity),
                    DamageType::Area { radius } => {
                        let shape = Collider::cylinder(radius, 10.0); // TODO: Height?
                        let shape_position = target_transform.translation;
                        let shape_rotation = Quat::default();

                        let mut hit_entities = spatial_query.shape_intersections(
                            &shape,
                            shape_position,
                            shape_rotation,
                            &SpatialQueryFilter::default(),
                        ); // TODO: filter
                        entities_to_damage.append(&mut hit_entities);
                    }
                }
            }
        } else {
            despawn = true;
        }

        for entity in entities_to_damage {
            if let Ok((_, _, mut health, enemy)) = targets.get_mut(entity) {
                // If the health is negative, this enemy was already killed by another projectile
                // and is already queued to be despawned
                if !health.0.is_sign_negative() {
                    health.0 -= projectile.damage;
                    if health.0.is_sign_negative() {
                        commands.entity(entity).despawn();
                        commands.spawn(sound_effect(audio_handles.enemy_death_from_time(&time)));
                        enemy_killed.write(EnemyKilled(enemy.definition.clone()));
                    }
                }
            }
        }

        if despawn {
            commands.entity(entity).despawn();
        }
    }
}
