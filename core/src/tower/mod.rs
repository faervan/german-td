use crate::{prelude::*, utils::on_ready_insert_animation_target};

mod placement;
pub use placement::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_plugins(placement::plugin(game_state));

        app.add_message::<SpawnTower>();

        app.add_systems(
            Update,
            (
                spawn_towers.run_if(on_message::<SpawnTower>),
                pick_tower_target,
                attack_tower_target,
                update_tower_targets,
            )
                .run_if(in_state(game_state)),
        );
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Tower {
    target: Option<Entity>,
    enemies_in_range: Vec<Entity>,
    attack_timer: Timer,
    projectile: Handle<ProjectileDefinition>,
    damage_factor: f32,
}

#[derive(Message, Debug)]
pub struct SpawnTower {
    pub position: Vec3,
    pub definition: Handle<TowerDefinition>,
    pub plot: Entity,
}

fn spawn_towers(
    mut gold: Option<ResMut<Gold>>,
    mut events: MessageReader<SpawnTower>,
    mut commands: Commands,
    definitions: Res<Assets<TowerDefinition>>,
    mut not_enough_gold: MessageWriter<NotEnoughGold>,
) {
    for spawn in events.read() {
        let def = definitions.get(&spawn.definition).unwrap();
        info!("Spawning tower {} at {:?}", def.name, spawn.position);

        if let Some(ref mut gold) = gold {
            // TODO: I am pretty sure we should not cast here; Make cost u32?
            let cost = def.cost as usize;
            if gold.0 < cost {
                info!("Not enough gold ({})!", cost);
                not_enough_gold.write(NotEnoughGold);
                return;
            }

            gold.0 -= cost;
        }

        let mut attack_timer = Timer::new(def.attack_duration, TimerMode::Repeating);
        attack_timer.finish();
        commands
            .spawn((
                Name::new(format!("Tower: {}", def.name)),
                Transform::from_translation(spawn.position),
                SceneRoot(def.scene.clone()),
                Tower {
                    target: None,
                    enemies_in_range: vec![],
                    attack_timer,
                    projectile: def.projectile.clone(),
                    damage_factor: def.damage_factor,
                },
                RigidBody::Static,
                Collider::sphere(def.range),
                CollisionLayers::new(GameLayer::Tower, GameLayer::Enemy),
                Sensor,
                CollisionEventsEnabled,
            ))
            .observe(on_ready_insert_animation_target);

        commands.entity(spawn.plot).despawn();
    }
}

#[inline]
/// When `e1` or `e2` is a tower, the other has to be an enemy as per the
/// [`GameLayer::Enemy`] filter on the towers [`CollisionLayers`]
fn get_tower_mut<'a>(
    towers: &'a mut Query<&mut Tower>,
    e1: Entity,
    e2: Entity,
) -> Option<(Entity, Mut<'a, Tower>)> {
    let (tower_entity, enemy_entity) = if towers.contains(e1) {
        (e1, e2)
    } else if towers.contains(e2) {
        (e2, e1)
    } else {
        return None;
    };
    Some((enemy_entity, towers.get_mut(tower_entity).unwrap()))
}

fn update_tower_targets(
    mut collision_starts: MessageReader<CollisionStart>,
    mut collision_ends: MessageReader<CollisionEnd>,
    mut towers: Query<&mut Tower>,
) {
    for event in collision_starts.read() {
        if let Some((enemy_entity, mut tower)) =
            get_tower_mut(&mut towers, event.collider1, event.collider2)
        {
            tower.enemies_in_range.push(enemy_entity);
        }
    }
    for event in collision_ends.read() {
        if let Some((enemy_entity, mut tower)) =
            get_tower_mut(&mut towers, event.collider1, event.collider2)
            && let Some(position) = tower
                .enemies_in_range
                .iter()
                .position(|e| *e == enemy_entity)
        {
            tower.enemies_in_range.swap_remove(position);
            if tower.target.is_some_and(|t| t == enemy_entity) {
                tower.target.take();
            }
        }
    }
}

/// Sets the target of the Tower Component
fn pick_tower_target(towers: Query<&mut Tower>) {
    for mut tower in towers {
        if tower.target.is_none() {
            // Not very sophisticated. Also this may not always be the enemy that has been in range
            // the longest, because we `swap_remove` enemies that go out-of-range, thus destroying
            // the order.
            // `VecDeque` could solve this, but the better solution would be adding more
            // interesting target picking mechanics imo (most forward enemy, fastest enemy, enemy
            // with most health, enemy with least health, etc.)
            tower.target = tower.enemies_in_range.first().copied();
        }
    }
}

fn attack_tower_target(
    time: Res<Time>,
    mut projectile_spawner: MessageWriter<SpawnProjectile>,
    mut towers: Query<(&mut Tower, &Transform)>,
) {
    for (mut tower, transform) in &mut towers {
        if tower.attack_timer.is_finished()
            && let Some(target) = tower.target
        {
            projectile_spawner.write(SpawnProjectile {
                position: transform.translation,
                target,
                definition: tower.projectile.clone(),
                damage_factor: tower.damage_factor,
            });
        }

        // tick timer
        tower.attack_timer.tick(time.delta());
    }
}
