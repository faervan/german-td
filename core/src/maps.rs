use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.init_resource::<Wave>();

        app.add_message::<SpawnMap>();
        app.add_message::<SpawnSpawners>();
        app.add_message::<StartWave>();

        app.add_systems(
            Update,
            spawn_maps
                .run_if(on_message::<SpawnMap>)
                .run_if(in_state(game_state)),
        );

        app.add_systems(
            Update,
            spawn_spawners
                .run_if(on_message::<SpawnSpawners>)
                .run_if(in_state(game_state)),
        );

        app.add_systems(
            Update,
            start_wave
                .run_if(on_message::<StartWave>)
                .run_if(in_state(game_state)),
        );

        app.add_systems(Update, spawn_from_spawner.run_if(in_state(game_state)));
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

/// Spawner per wave
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Spawner {
    position: Vec3,
    spawns: Vec<(Duration, Handle<EnemyDefinition>)>,
    elapsed: Duration,
}

impl Spawner {
    pub fn new(position: Vec3, spawns: Vec<(Duration, Handle<EnemyDefinition>)>) -> Self {
        Self {
            position,
            spawns,
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
        if let Some(spawn) = spawner.spawns.last().cloned() {
            spawner.elapsed += time.delta();

            if spawner.elapsed > spawn.0 {
                spawn_enemy.write(SpawnEnemy {
                    position: spawner.position,
                    definition: spawn.1.clone(),
                });

                spawner.elapsed -= spawn.0;

                spawner.spawns.pop();
            }
        }
    }
}

#[derive(Debug, Message, Reflect)]
pub struct SpawnSpawners {
    map_definition: Handle<MapDefinition>,
    wave: usize,
}

fn spawn_spawners(
    mut events: MessageReader<SpawnSpawners>,
    mut commands: Commands,
    definitions: Res<Assets<MapDefinition>>,
) {
    for spawn in events.read() {
        let map_definition = definitions.get(&spawn.map_definition).unwrap();
        info!(
            "Spawning spawners for {} wave {}",
            map_definition.name, spawn.wave
        );

        for (i, path) in map_definition.paths.iter().enumerate() {
            let position = map_definition
                .waypoints
                .get(*path.waypoints.first().unwrap())
                .unwrap();
            let spawns = path.spawner.spawns.get(spawn.wave).unwrap();

            commands.spawn((
                Name::new(format!("Spawner {} at {}", i, position)),
                Spawner::new(*position, spawns.clone()),
            ));
        }
    }
}

// This is akward

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Wave(pub usize);

#[derive(Debug, Message, Reflect)]
pub struct StartWave;

fn start_wave(
    mut start_wave: MessageReader<StartWave>,
    maps: Query<&Map>,
    mut spawn_spawners: MessageWriter<SpawnSpawners>,
    wave: Res<Wave>,
) {
    for _ in start_wave.read() {
        for map in &maps {
            spawn_spawners.write(SpawnSpawners {
                map_definition: map.definition.clone(),
                wave: wave.0,
            });
        }
    }
}
