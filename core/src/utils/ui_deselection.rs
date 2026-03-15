use crate::prelude::*;

pub(crate) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.init_resource::<FocusedUi>();

        app.add_observer(on_click);
        app.add_systems(PostUpdate, update_focused.run_if(in_state(game_state)));
    }
}

#[derive(Resource, Default)]
/// Tracks focused entities and despawns them when something else is clicked.
pub struct FocusedUi {
    focused: HashSet<Entity>,
    clicked: HashSet<Entity>,
    received_click: bool,
}

impl FocusedUi {
    /// Register the given [`Entity`] as focused. It has to be marked as clicked in every frame a
    /// click occur ed, else it will be despawned.
    pub fn register_focus(&mut self, entity: Entity) {
        self.focused.insert(entity);
        self.clicked.insert(entity);
    }

    /// Mark the given [`Entity`] as clicked this frame
    pub fn register_click(&mut self, entity: Entity) {
        self.clicked.insert(entity);
    }
}

fn on_click(_event: On<Pointer<Click>>, mut focused_ui: ResMut<FocusedUi>) {
    focused_ui.received_click = true;
}

fn update_focused(mut focused_ui: ResMut<FocusedUi>, mut commands: Commands) {
    if !focused_ui.received_click {
        focused_ui.clicked = HashSet::new();
        return;
    }

    for entity in focused_ui
        .focused
        .iter()
        .filter(|e| !focused_ui.clicked.contains(e))
    {
        commands.entity(*entity).try_despawn();
    }

    focused_ui.clicked = HashSet::new();
    focused_ui.received_click = false;
}
