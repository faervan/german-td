use crate::{prelude::*, utils::on_ready_insert_animation_target};

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_message::<SpawnEnemy>();

        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_message::<SpawnEnemy>),
                manage_controllers,
            )
                .run_if(in_state(game_state)),
        );
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Enemy {
    pub definition: Handle<EnemyDefinition>,
}

#[derive(Message, Debug)]
pub struct SpawnEnemy {
    pub position: Vec3,
    pub definition: Handle<EnemyDefinition>,
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
                Health(def.health),
                // Physics
                RigidBody::Kinematic,
                Collider::cylinder(0.3, 1.5),
                GravityScale(0.),
            ))
            .observe(on_ready_insert_animation_target);
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct EnemyController {
    start_moving: bool,
    stop_moving: bool,
    /// Managed by the controller
    moving: bool,

    start_attack: bool,
    /// Managed by the controller
    attacking: Option<Timer>,
}

impl EnemyController {
    /// Attempt to start an attack, returning `false` if an attack is already in process
    pub fn attack(&mut self) -> bool {
        let already_attacking = self.attacking.is_some() || self.start_attack;
        if !already_attacking {
            self.start_attack = true;
        }
        !already_attacking
    }

    /// Attempt to start moving, returning `false` if already moving
    pub fn start_moving(&mut self) -> bool {
        let already_moving = self.moving || self.start_moving;
        if !already_moving {
            self.start_moving = true;
        }
        !already_moving
    }

    /// Attempt to stop moving, returning `false` if the enemy was not moving
    pub fn stop_moving(&mut self) -> bool {
        let already_idle = !self.moving || self.stop_moving;
        if !already_idle {
            self.stop_moving = true;
        }
        !already_idle
    }
}

fn manage_controllers(
    time: Res<Time>,
    definitions: Res<Assets<EnemyDefinition>>,
    query: Query<(&Enemy, &mut EnemyController, &GltfAnimationTarget)>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    for (enemy, mut controller, animation_target) in query {
        let mut animation = None;
        let mut repeat = false;

        if controller.start_attack {
            let def = definitions.get(&enemy.definition).unwrap();
            controller.attacking = Some(Timer::new(def.attack_duration, TimerMode::Once));
            controller.start_attack = false;
            animation = def.attack_animation.as_ref();
        } else if let Some(timer) = &mut controller.attacking {
            timer.tick(time.delta());
            if timer.just_finished() {
                controller.attacking = None;

                let def = definitions.get(&enemy.definition).unwrap();
                match controller.moving {
                    true => {
                        animation = def.walk_animation.as_ref();
                        repeat = true;
                    }
                    false => animation = def.idle_animation.as_ref(),
                }
            }
        }

        if controller.start_moving {
            let def = definitions.get(&enemy.definition).unwrap();
            controller.start_moving = false;
            controller.moving = true;
            animation = def.walk_animation.as_ref();
            repeat = true;
        }
        if controller.stop_moving {
            let def = definitions.get(&enemy.definition).unwrap();
            controller.stop_moving = false;
            controller.moving = false;
            animation = def.idle_animation.as_ref();
        }

        if let Some(Ok(animation)) = animation
            && let Ok((mut animation_player, mut animation_transitions)) =
                animation_players.get_mut(animation_target.0)
        {
            let active = animation_transitions.play(
                &mut animation_player,
                *animation,
                Duration::from_millis(100),
            );
            if repeat {
                active.repeat();
            }
        }
    }
}
