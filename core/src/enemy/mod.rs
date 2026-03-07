use crate::{
    enemy::controller::EnemyController, prelude::*, utils::on_ready_insert_animation_target,
};

mod controller;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_message::<EnemyReachedGoal>();
        app.add_message::<SpawnEnemy>();

        app.add_plugins(controller::plugin(game_state));

        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_message::<SpawnEnemy>),
                enemy_follow_path,
            )
                .run_if(in_state(game_state)),
        );

        // TODO! remove, testing only
        app.add_systems(Update, enemy_ctrl.run_if(in_state(game_state)));
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Enemy {
    pub definition: Handle<EnemyDefinition>,
}

#[derive(Message, Debug)]
pub struct EnemyReachedGoal {
    pub definition: Handle<EnemyDefinition>,
    pub entity: Entity,
}

#[derive(Message, Debug)]
pub struct SpawnEnemy {
    pub position: Vec3,
    pub definition: Handle<EnemyDefinition>,
    /// The last item is the one the enemy is currently heading towards
    pub waypoints: Arc<Vec<Vec3>>,
}

fn spawn_enemies(
    mut events: MessageReader<SpawnEnemy>,
    mut commands: Commands,
    definitions: Res<Assets<EnemyDefinition>>,
) {
    for spawn in events.read() {
        let def = definitions.get(&spawn.definition).unwrap();
        info!("Spawning enemy {} at {:?}", def.name, spawn.position);

        commands
            .spawn((
                Name::new(format!("Enemy: {}", def.name)),
                Transform::from_translation(spawn.position),
                SceneRoot(def.scene.clone()),
                Enemy {
                    definition: spawn.definition.clone(),
                },
                EnemyController::default(),
                EnemyFollowPath {
                    waypoints: spawn.waypoints.clone(),
                    current: 0,
                },
                Health(def.health),
                // Physics
                RigidBody::Kinematic,
                Collider::cylinder(0.3, 1.5),
                CollisionLayers::new(GameLayer::Enemy, GameLayer::all_bits()),
                GravityScale(0.),
            ))
            .observe(on_ready_insert_animation_target);
    }
}

fn enemy_ctrl(input: Res<ButtonInput<KeyCode>>, mut controllers: Query<&mut EnemyController>) {
    if input.just_pressed(KeyCode::KeyH) {
        for mut controller in &mut controllers {
            if !controller.attack() {
                warn!("Already attacking!");
            }
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct EnemyFollowPath {
    waypoints: Arc<Vec<Vec3>>,
    /// Index of the current waypoint
    current: usize,
}

fn enemy_follow_path(
    mut goal_reached: MessageWriter<EnemyReachedGoal>,
    time: Res<Time>,
    mut commands: Commands,
    enemy_defs: Res<Assets<EnemyDefinition>>,
    query: Query<(
        Entity,
        &Enemy,
        &mut EnemyController,
        &mut Transform,
        &mut EnemyFollowPath,
    )>,
) {
    for (entity, enemy, mut controller, mut transform, mut follow_path) in query {
        let Some(waypoint) = follow_path.waypoints.get(follow_path.current) else {
            commands.entity(entity).remove::<EnemyFollowPath>();
            controller.stop_moving();
            goal_reached.write(EnemyReachedGoal {
                definition: enemy.definition.clone(),
                entity,
            });
            continue;
        };

        let Some(enemy_def) = enemy_defs.get(&enemy.definition) else {
            continue;
        };
        let direction = (waypoint - transform.translation).normalize_or_zero();
        transform.translation += direction * enemy_def.walk_speed * time.delta_secs();

        controller.start_moving();

        if waypoint.distance(transform.translation) < 1. {
            follow_path.current += 1;
            if let Some(new_waypoint) = follow_path.waypoints.get(follow_path.current) {
                commands.entity(entity).animate_towards(
                    transform.looking_at(*new_waypoint, Vec3::Y),
                    Duration::from_millis(100),
                );
            }
        }
    }
}
