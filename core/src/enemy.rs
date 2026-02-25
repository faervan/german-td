use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_message::<SpawnEnemy>();

        app.add_systems(
            Update,
            (spawn_enemies, manage_controllers).run_if(in_state(game_state)),
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

        commands.spawn((
            Name::new(format!("Enemy: {}", def.name)),
            Transform::from_translation(spawn.position),
            SceneRoot(def.scene.clone()),
            Enemy {
                definition: spawn.definition.clone(),
            },
            EnemyController::default(),
            // Physics
            RigidBody::Kinematic,
            Collider::cylinder(0.3, 1.5),
            GravityScale(0.),
        ));
    }
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct EnemyController {
    moving: bool,
    start_attack: bool,
    attacking: Option<Timer>,
}

impl EnemyController {
    /// Attempt to start an attack, returning `false` if an attack is already in process
    pub fn attack(&mut self) -> bool {
        let already_attacking = self.attacking.is_some() || self.start_attack;
        if !already_attacking {
            self.start_attack = true;
        }
        already_attacking
    }

    pub fn start_moving(&mut self) {
        self.moving = true;
    }

    pub fn stop_moving(&mut self) {
        self.moving = false;
    }
}

fn manage_controllers(
    time: Res<Time>,
    definitions: Res<Assets<EnemyDefinition>>,
    query: Query<(&Enemy, &mut EnemyController)>,
) {
    for (enemy, mut controller) in query {
        if controller.start_attack {
            let def = definitions.get(&enemy.definition).unwrap();
            controller.attacking = Some(Timer::new(def.attack_duration, TimerMode::Once));
            controller.start_attack = false;
        } else if let Some(timer) = &mut controller.attacking {
            timer.tick(time.delta());
            if timer.just_finished() {
                controller.attacking = None;
            }
        }
    }
}
