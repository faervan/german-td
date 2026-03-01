use crate::{assets::maps::EnemyPath, prelude::*};

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_message::<SpawnMap>();

        app.add_systems(
            Update,
            spawn_maps
                .run_if(on_message::<SpawnMap>)
                .run_if(in_state(game_state)),
        );

        app.add_systems(Update, spawn_from_spawner);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Map {
    pub definition: Handle<MapDefinition>,
}

#[derive(Message, Debug)]
pub struct SpawnMap {
    pub definition: Handle<MapDefinition>,
}

fn spawn_maps(
    mut events: MessageReader<SpawnMap>,
    mut commands: Commands,
    definitions: Res<Assets<MapDefinition>>,
) {
    for spawn in events.read() {
        let def = definitions.get(&spawn.definition).unwrap();
        info!("Spawning map {}", def.name);

        commands.spawn((
            Name::new(format!("Map: {}", def.name)),
            Transform::default(),
            SceneRoot(def.scene.clone()),
            Map {
                definition: spawn.definition.clone(),
            },
        ));
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Spawner {
    position: Vec3,
    enemy_path: EnemyPath,
    current_spawn: Option<(Duration, Handle<EnemyDefinition>)>,
    current_wave: usize,
    elapsed: Duration,
}

impl Spawner {
    pub fn new(enemy_path: EnemyPath, position: Vec3) -> Self {
        Self {
            position,
            enemy_path,
            current_spawn: None,
            current_wave: 0,
            elapsed: Duration::ZERO,
        }
    }
}

fn spawn_from_spawner(
    time: Res<Time>,
    mut spawners: Query<&mut Spawner>,
    mut spawn_enemy: MessageWriter<SpawnEnemy>,
) {
    for mut spawner in &mut spawners {
        let current_wave = spawner.current_wave;
        let mut current_spawn = spawner.current_spawn.clone();
        let mut elapsed = spawner.elapsed;
        let position = spawner.position;

        if let Some(current_wave) = spawner.enemy_path.spawner.spawns.get_mut(current_wave) {
            if current_spawn.is_none() {
                current_spawn = current_wave.pop();
            }

            elapsed += time.delta();

            if let Some(ref spawn) = current_spawn
                && elapsed >= spawn.0
            {
                spawn_enemy.write(SpawnEnemy {
                    position,
                    definition: spawn.1.clone(),
                });

                elapsed -= spawn.0;

                current_spawn = current_wave.pop();
            }
        }

        spawner.current_spawn = current_spawn;
        spawner.elapsed = elapsed;
    }
}
