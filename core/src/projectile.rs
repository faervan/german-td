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
    damage: f32,
    damage_type: DamageType,
}

#[derive(Message, Debug)]
pub struct SpawnProjectile {
    pub position: Vec3,
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
    projectile_transforms: Query<(
        Entity,
        &Projectile,
        &GlobalTransform,
        &mut Transform,
        &mut LinearVelocity,
    )>,
    mut targets: Query<(Entity, &Enemy, &GlobalTransform, &mut Health), Without<Projectile>>,
    time: Res<Time>,
    audio_handles: Res<GameSoundHandles>,
    mut enemy_killed: MessageWriter<EnemyKilled>,
    spatial_query: SpatialQuery,
) {
    for (
        entity,
        projectile,
        projectile_global_transform,
        mut projectile_transform,
        mut projectile_velocity,
    ) in projectile_transforms
    {
        let mut despawn = false;
        let mut entities_to_damage = vec![];

        if let Some(target_position) = match projectile.damage_type {
            DamageType::Single { target } => {
                if let Ok((_, _, target_transform, _)) = targets.get(target) {
                    Some(target_transform.translation())
                } else {
                    despawn = true;
                    None
                }
            }
            DamageType::Area { target_pos, .. } => Some(target_pos),
        } {
            let direction = target_position - projectile_global_transform.translation();

            projectile_transform.look_at(target_position, Vec3::Y);
            projectile_velocity.0 = direction.normalize() * 30.0;

            if direction.length() < 1.0 {
                despawn = true;

                match projectile.damage_type {
                    DamageType::Single { target } => entities_to_damage.push(target),
                    DamageType::Area { radius, .. } => {
                        let shape = Collider::cylinder(radius, 100.0); // TODO: height?
                        let shape_position = target_position;
                        let shape_rotation = Quat::default();

                        let mut hit_entities = spatial_query.shape_intersections(
                            &shape,
                            shape_position,
                            shape_rotation,
                            &SpatialQueryFilter::default(),
                        ); // TODO: filter
                        info!("Hit entities: {:?}", hit_entities);
                        entities_to_damage.append(&mut hit_entities);
                    }
                }
            }
        }

        for entity in entities_to_damage {
            if let Ok((_, enemy, _, mut health)) = targets.get_mut(entity) {
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
