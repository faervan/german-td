use bevy::ecs::system::RunSystemOnce as _;
use german_td_core::{asset_plugin, default_plugins};

mod prelude;
use prelude::*;

mod camera;
mod cursor;
mod editor_ui;
mod enemy;
mod focus;
mod map;
mod preview;
mod spawn_menu;
mod tower;

fn main() {
    let mut app = App::new();

    // Bevy config
    app.add_plugins(DefaultPlugins.set(asset_plugin()).set(WindowPlugin {
        primary_window: Some(Window {
            title: "German TD Editor".into(),
            name: Some("german_td_editor".into()),
            ..Default::default()
        }),
        ..Default::default()
    }));

    app.add_plugins((
        default_plugins(State::Loading, State::Editor),
        camera::plugin,
        cursor::plugin,
        editor_ui::plugin,
        tower::plugin,
        focus::plugin,
        map::plugin,
        preview::plugin,
        spawn_menu::plugin,
    ));

    app.init_state::<State>();

    app.add_systems(
        Update,
        set_editor_state.run_if(in_state(State::Loading).and(all_assets_loaded)),
    );

    app.add_systems(
        Update,
        toggle_aabb_gizmo.run_if(
            in_state(State::Editor)
                .and(input_pressed(KeyCode::AltLeft))
                .and(input_just_pressed(KeyCode::KeyG)),
        ),
    );

    app.add_systems(
        Update,
        save.run_if(input_pressed(KeyCode::ControlLeft).and(input_just_pressed(KeyCode::KeyS))),
    );
    app.add_systems(
        Update,
        (save, and_exit)
            .chain()
            .run_if(input_pressed(KeyCode::ControlLeft).and(input_just_pressed(KeyCode::KeyQ))),
    );

    app.run();
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum State {
    #[default]
    Loading,
    Editor,
}

fn set_editor_state(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::Editor);
}

fn toggle_aabb_gizmo(mut config: ResMut<GizmoConfigStore>) {
    let (_, config) = config.config_mut::<AabbGizmoConfigGroup>();
    config.draw_all = !config.draw_all;
}

fn save(world: &mut World) {
    if let Err(e) = world.run_system_once(map::save) {
        error!("Failed to save map: {e}");
    }

    let mut manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.pop();
    let Some(manifest_dir_str) = manifest_dir.to_str() else {
        error!("{} cannot be converted to str", manifest_dir.display());
        return;
    };
    let asset_dir = PathBuf::from_iter([manifest_dir_str, "assets"]);

    world.resource_scope(|world, script_defs: Mut<Assets<ScriptAsset>>| {
        //
        // Save maps
        //
        let map_strings = world
            .resource_mut::<Assets<MapDefinition>>()
            .iter_mut()
            .filter_map(|(_, def)| def.serialize(&script_defs).ok())
            .collect::<Vec<_>>();
        for (name, serialized_string) in map_strings {
            let path = asset_dir.join(MapDefinition::path(&name));
            info!("Saving map {name} to {}", path.display());
            if let Err(e) = std::fs::write(path, serialized_string) {
                error!("Saving failed: {e}");
            }
        }
        //
        // Save scripts
        //
        for (_, script) in script_defs.iter() {
            let path = asset_dir.join(PathBuf::from(&script.file));
            info!("Saving script {}", &script.file);
            if let Err(e) = std::fs::write(&path, &script.source) {
                error!("Failed to save script {}: {e}", path.display());
            }
        }
        //
        // Save towers
        //
        let tower_strings = world
            .resource_mut::<Assets<TowerDefinition>>()
            .iter_mut()
            .filter_map(|(_, def)| def.serialize().ok())
            .collect::<Vec<_>>();
        for (name, serialized_string) in tower_strings {
            let path = asset_dir.join(TowerDefinition::path(&name));
            info!("Saving tower {name} to {}", path.display());
            if let Err(e) = std::fs::write(path, serialized_string) {
                error!("Saving failed: {e}");
            }
        }
    });
}

fn and_exit(world: &mut World) {
    world.write_message(AppExit::Success);
}
