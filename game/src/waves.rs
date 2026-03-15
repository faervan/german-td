use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        start_wave_spawning.run_if(on_message::<SpawnMap>.and(in_state(AppState::Game))),
    );

    app.add_systems(OnEnter(AppState::Game), setup_wave_ui);
    app.add_systems(
        Update,
        update_wave_ui
            .run_if(in_state(AppState::Game))
            .run_if(resource_exists::<WaveSpawning>),
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

#[derive(Component)]
struct WaveText;

#[derive(Component)]
struct WaveTimerText;

fn setup_wave_ui(mut commands: Commands) {
    let root = commands
        .spawn((
            Name::new("Wave UI root"),
            Node {
                width: percent(100),
                height: percent(100),
                ..Default::default()
            },
            DespawnOnExit(AppState::Game),
        ))
        .id();

    let ui = commands
        .spawn((
            Name::new("Wave UI"),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(10),
                left: px(10),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::Start,
                row_gap: px(5),
                ..Default::default()
            },
            ChildOf(root),
        ))
        .id();

    commands.spawn((Name::new("WaveText"), WaveText, Text::new(""), ChildOf(ui)));

    commands.spawn((
        Name::new("WaveTimerText"),
        WaveTimerText,
        Text::new(""),
        ChildOf(ui),
    ));
}

fn update_wave_ui(
    wave: Res<WaveSpawning>,
    mut wave_text: Single<&mut Text, With<WaveText>>,
    mut wave_timer_text: Single<&mut Text, (With<WaveTimerText>, Without<WaveText>)>,
) {
    wave_text.0 = if wave.current > 0 {
        format!("Wave {} of {}", wave.current, wave.last)
    } else {
        String::new()
    };

    wave_timer_text.0 = if wave.active_spawners == 0
        && let Some(timer) = &wave.cooldown
    {
        format!(
            "Wave {} starts in {} seconds",
            wave.current + 1,
            timer.remaining_secs().round()
        )
    } else if wave.cooldown.is_some() || wave.active_spawners > 0 {
        format!(
            "{} spawner{} active",
            wave.active_spawners,
            if wave.active_spawners == 1 { "" } else { "s" }
        )
    } else {
        String::from("All waves complete")
    };
}
