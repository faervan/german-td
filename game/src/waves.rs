use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        start_wave_spawning.run_if(on_message::<SpawnMap>.and(in_state(AppState::Game))),
    );
}

fn start_wave_spawning(
    mut commands: Commands,
    mut events: MessageReader<SpawnMap>,
    map_defs: Res<Assets<MapDefinition>>,
) {
    for spawn in events.read() {
        if let Some(def) = map_defs.get(&spawn.definition) {
            commands.insert_resource(WaveSpawning {
                current: 0,
                last: def.waves(),
                active_spawners: 0,
                cooldown: Some(Timer::new(Duration::from_secs(5), TimerMode::Once)),
            });
        }
    }
}
