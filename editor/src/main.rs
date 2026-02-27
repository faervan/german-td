use german_td_core::{asset_plugin, default_plugins};

mod prelude;
use prelude::*;

mod camera;
mod gui;
mod preview;

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
        gui::plugin,
        preview::plugin,
        camera::plugin,
    ));

    app.init_state::<State>();

    app.add_systems(
        Update,
        set_editor_state.run_if(in_state(State::Loading).and(all_assets_loaded)),
    );

    app.add_systems(
        Update,
        exit_game
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

fn exit_game(mut exit: MessageWriter<AppExit>) {
    exit.write(AppExit::Success);
}
