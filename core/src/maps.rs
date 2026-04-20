use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_message::<SpawnMap>();
        app.add_message::<StartWave>();

        app.add_systems(
            Update,
            spawn_maps
                .run_if(on_message::<SpawnMap>)
                .run_if(in_state(game_state)),
        );

        app.add_systems(
            Update,
            start_wave
                .run_if(resource_exists::<WaveSpawning>)
                .run_if(on_message::<StartWave>)
                .run_if(in_state(game_state)),
        );

        app.add_systems(
            Update,
            spawn_from_spawner
                .run_if(resource_exists::<WaveSpawning>)
                .run_if(in_state(game_state)),
        );
        app.add_systems(
            Update,
            advance_wave_spawning
                .run_if(resource_exists::<WaveSpawning>)
                .run_if(in_state(game_state)),
        );
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
    #[cfg(not(feature = "editor"))] mut placement_spawner: MessageWriter<SpawnTowerPlacement>,
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

        #[cfg(not(feature = "editor"))]
        for position in def.tower_plots.clone() {
            placement_spawner.write(SpawnTowerPlacement { position });
        }
    }
}

/// Spawner per wave
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Spawner {
    pub position: Vec3,
    spawns: Vec<(Duration, Handle<EnemyDefinition>)>,
    /// The path that spawned enemies will follow
    pub waypoints: Arc<Vec<Vec3>>,
    elapsed: Duration,
}

impl Spawner {
    fn new(
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
    mut wave: ResMut<WaveSpawning>,
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
            wave.active_spawners -= 1;
        }
    }
}

#[derive(Debug, Message, Reflect)]
struct StartWave(usize);

fn start_wave(
    mut start_wave: MessageReader<StartWave>,
    mut wave_spawning: ResMut<WaveSpawning>,
    maps: Query<&Map>,
    mut commands: Commands,
    definitions: Res<Assets<MapDefinition>>,
    mut scripts: ResMut<Assets<ScriptAsset>>,
    enemy_lib: EnemyLibrary,
) {
    for wave in start_wave.read() {
        if let Ok(map) = maps.single() {
            let map_definition = definitions.get(&map.definition).unwrap();
            info!(
                "Spawning spawners for {} wave {}",
                map_definition.name(),
                wave.0
            );

            wave_spawning.active_spawners = map_definition.paths.len();

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
                    .call(wave.0 as u32, scripting::Val(enemy_lib.clone()))
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
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct WaveSpawning {
    pub current: usize,
    pub last: usize,
    /// Spawners spawned by the current wave that are still spawning new enemies
    pub active_spawners: usize,
    /// Cooldown before and between waves
    pub cooldown: Option<Timer>,
}

fn advance_wave_spawning(
    time: Res<Time>,
    mut wave: ResMut<WaveSpawning>,
    mut wave_spawner: MessageWriter<StartWave>,
) {
    if wave.active_spawners == 0
        && let Some(timer) = wave.cooldown.as_mut()
    {
        timer.tick(time.delta());
        if timer.is_finished() {
            timer.reset();
            if wave.current < wave.last {
                wave.current += 1;
                wave_spawner.write(StartWave(wave.current));
                if wave.current == wave.last {
                    wave.cooldown = None;
                }
            } else {
                debug!("All waves finished!");
                wave.cooldown = None;
            }
        }
    }
}
