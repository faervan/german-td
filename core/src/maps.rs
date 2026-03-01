use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_message::<SpawnMap>();

        app.add_systems(
            Update,
            spawn_maps
                .run_if(on_message::<SpawnMap>)
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
#[component(on_add)]
#[reflect(Component)]
pub struct Spawner {
    map_definition: Handle<MapDefinition>,
    path_index: usize,
    spawn_index: usize,
    timer: Timer,
}

impl Spawner {
    pub fn new(map_definition: Handle<MapDefinition>, path_index: usize) -> Self {
        Self {
            map_definition,
            path_index,
            spawn_index: 0,
            timer: Default::default(),
        }
    }

    // Set up timer
    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let spawner = world.get::<Self>(ctx.entity).expect("Added component is unavailable in on_add hook");
        let mut timer = Timer::default();

        if let Some(map_definitions) = world.get_resource::<Assets<MapDefinition>>() {
            if let Some(map_definition) = map_definitions.get(&spawner.map_definition) {
                if let Some(path) = map_definition.paths.get(spawner.path_index) {
                    if let Some(first_wave) = path.spawner.spawns.first() {
                        if let Some((duration, _)) = first_wave.last() {
                            timer = Timer::new(*duration, TimerMode::Once);
                        }
                    }
                }
            }
        }

        let mut spawner = world.get_mut::<Self>(ctx.entity).expect("Added component is unavailable in on_add hook");
        spawner.timer = timer;
    }
}

fn spawn_from_spawner(time: Res<Time>, mut spawners: Query<&mut Spawner>, map_definitions: Res<Assets<MapDefinition>>) {
    for mut spawner in &mut spawners {
        if spawner.timer.tick(time.delta()).just_finished() {
            // Spawn enemy
            if let Some(map_definition) = map_definitions.get(&spawner.map_definition) {
                if let Some(path) = map_definition.paths.get(spawner.path_index) {
                    if let Some(first_wave) = path.spawner.spawns.first() {
                        if let Some(spawn) = first_wave.iter().nth_back(spawner.spawn_index) {
                            
                        }
                    }
                }
            }

            spawner.spawn_index += 1;
        }
    }
}