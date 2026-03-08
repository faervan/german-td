use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.insert_resource(Wave(1));

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
    mut placement_spawner: MessageWriter<SpawnTowerPlacement>,
    mut commands: Commands,
    definitions: Res<Assets<MapDefinition>>,
) {
    for spawn in events.read() {
        let def = definitions.get(&spawn.definition).unwrap();
        info!("Spawning map {}", def.name());

        commands.spawn((
            Name::new(format!("Map: {}", def.name())),
            Transform::default(),
            SceneRoot(def.scene.clone()),
            Map {
                definition: spawn.definition.clone(),
            },
        ));

        for position in def.tower_plots.clone() {
            placement_spawner.write(SpawnTowerPlacement { position });
        }
    }
}

/// Spawner per wave
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Spawner {
    position: Vec3,
    spawns: Vec<(Duration, Handle<EnemyDefinition>)>,
    /// The path that spawned enemies will follow
    waypoints: Arc<Vec<Vec3>>,
    elapsed: Duration,
}

impl Spawner {
    pub fn new(
        position: Vec3,
        spawns: Vec<(Duration, Handle<EnemyDefinition>)>,
        waypoints: Vec<Vec3>,
    ) -> Self {
        Self {
            position,
            spawns,
            waypoints: Arc::new(waypoints),
            elapsed: Duration::ZERO,
        }
    }
}

fn spawn_from_spawner(
    time: Res<Time>,
    mut commands: Commands,
    mut spawners: Query<(Entity, &mut Spawner)>,
    mut spawn_enemy: MessageWriter<SpawnEnemy>,
) {
    for (spawner_entity, mut spawner) in &mut spawners {
        if let Some(spawn) = spawner.spawns.last().cloned() {
            spawner.elapsed += time.delta();

            if spawner.elapsed > spawn.0 {
                spawn_enemy.write(SpawnEnemy {
                    position: spawner.position,
                    definition: spawn.1.clone(),
                    waypoints: spawner.waypoints.clone(),
                });

                spawner.elapsed -= spawn.0;

                spawner.spawns.pop();
            }
        } else {
            commands.entity(spawner_entity).despawn();
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
    mut scripts: ResMut<Assets<ScriptAsset>>,
    enemy_lib: EnemyLibrary,
) {
    for spawn in events.read() {
        let map_definition = definitions.get(&spawn.map_definition).unwrap();
        info!(
            "Spawning spawners for {} wave {}",
            map_definition.name(),
            spawn.wave
        );

        for (i, path) in map_definition.paths.iter().enumerate() {
            let position = map_definition.waypoints().get(path.waypoints[0]).unwrap();
            let Some(spawn_function) = path.spawner.get_spawner_function(&mut scripts) else {
                warn!(
                    "Failed to get spawn function for path {i} of {}",
                    map_definition.name()
                );
                continue;
            };
            let spawns: Vec<_> = spawn_function
                .call(spawn.wave as u32, scripting::Val(enemy_lib.clone()))
                .to_vec()
                .into_iter()
                .map(|val| val.0)
                .collect();

            let waypoints = path
                .waypoints
                .iter()
                .map(|waypoint_id| map_definition.waypoints()[*waypoint_id])
                .collect();

            commands.spawn((
                Name::new(format!("Spawner {} at {}", i, position)),
                Spawner::new(*position, spawns.clone(), waypoints),
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
    map_defs: Res<Assets<MapDefinition>>,
    mut spawn_spawners: MessageWriter<SpawnSpawners>,
    mut wave: ResMut<Wave>,
) {
    for _ in start_wave.read() {
        if let Ok(map) = maps.single()
            && let Some(map_def) = map_defs.get(&map.definition)
        {
            spawn_spawners.write(SpawnSpawners {
                map_definition: map.definition.clone(),
                wave: wave.0,
            });
            wave.0 += 1;
            if wave.0 > map_def.waves() {
                info!(
                    "Finished all {} waves, restarting at first wave",
                    map_def.waves()
                );
                wave.0 = 1;
            }
        }
    }
}
