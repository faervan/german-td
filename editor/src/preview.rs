use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(State::Editor), setup);
}

fn setup(mut map_spawner: MessageWriter<SpawnMap>, map_lib: MapLibrary) {
    map_spawner.write(SpawnMap {
        definition: map_lib.entries["First"].clone(),
    });
}
