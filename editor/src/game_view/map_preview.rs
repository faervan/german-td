use crate::{map::SpawnEnemyPaths, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(State::Editor), setup);

    app.add_systems(Update, spawn_map_items.run_if(in_state(State::Editor)));
}

fn setup(mut map_spawner: MessageWriter<SpawnMap>, map_lib: MapLibrary) {
    map_spawner.write(SpawnMap {
        definition: map_lib.entries["First"].clone(),
    });
}

fn spawn_map_items(
    mut waypoints: MessageWriter<SpawnWaypoint>,
    mut plots: MessageWriter<SpawnTowerPlot>,
    mut paths: MessageWriter<SpawnEnemyPaths>,
    maps: Res<Assets<MapDefinition>>,
    query: Query<&Map, Added<Map>>,
) {
    for map in query {
        if let Some(definition) = maps.get(&map.definition) {
            for waypoint in definition.waypoints() {
                waypoints.write(SpawnWaypoint {
                    position: Some(*waypoint),
                });
            }

            for plot in &definition.tower_plots {
                plots.write(SpawnTowerPlot {
                    position: Some(*plot),
                });
            }

            paths.write(SpawnEnemyPaths {
                map_definition: map.definition.clone(),
            });
        }
    }
}
