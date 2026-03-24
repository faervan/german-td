use bevy::color::palettes::css::{GOLD, RED};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gold(20));

    app.add_systems(OnEnter(AppState::Game), setup_gold_ui);

    app.add_systems(Update, update_gold_ui.run_if(in_state(AppState::Game)));

    app.add_systems(
        Update,
        update_not_enough_gold_text_ui
            .run_if(on_message::<NotEnoughGold>.and(in_state(AppState::Game))),
    );
    app.add_systems(
        Update,
        update_not_enough_gold_text_ui_tick.run_if(in_state(AppState::Game)),
    );
    app.add_systems(
        Update,
        gain_gold_from_enemy.run_if(on_message::<EnemyKilled>.and(in_state(AppState::Game))),
    );
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct GoldText;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct NotEnoughGoldText(Timer);

fn setup_gold_ui(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Gold UI root"),
            Node {
                width: percent(100),
                height: percent(100),
                ..Default::default()
            },
            DespawnOnExit(AppState::Game),
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("GoldText"),
                Text::new(""),
                TextColor(GOLD.into()),
                GoldText,
            ));

            parent.spawn((
                Name::new("NotEnoughGoldText"),
                Text::new(""),
                TextColor(RED.into()),
                NotEnoughGoldText(Timer::default()),
            ));
        });
}

fn update_gold_ui(gold: Res<Gold>, mut gold_text: Single<&mut Text, With<GoldText>>) {
    gold_text.0 = format!("Gold: {}", gold.0);
}

fn update_not_enough_gold_text_ui(
    mut not_enough_gold: MessageReader<NotEnoughGold>,
    mut not_enough_gold_text: Single<(&mut Text, &mut NotEnoughGoldText)>,
) {
    for _not_enough_gold in not_enough_gold.read() {
        let (ref mut text, ref mut not_enough_gold_text) = *not_enough_gold_text;
        text.0 = String::from("NOT ENOUGH GOLD");

        not_enough_gold_text.0 = Timer::new(Duration::from_secs_f32(1.5), TimerMode::Once);
    }
}

fn update_not_enough_gold_text_ui_tick(
    time: Res<Time>,
    mut not_enough_gold_text: Single<(&mut NotEnoughGoldText, &mut TextColor)>,
) {
    let (ref mut not_enough_gold_text, ref mut color) = *not_enough_gold_text;

    not_enough_gold_text.0.tick(time.delta());

    color
        .0
        .set_alpha(not_enough_gold_text.0.fraction_remaining());
}

fn gain_gold_from_enemy(
    mut gold: ResMut<Gold>,
    mut enemy_killed: MessageReader<EnemyKilled>,
    enemies: Res<Assets<EnemyDefinition>>,
) {
    for enemy_killed in enemy_killed.read() {
        if let Some(definition) = enemies.get(&enemy_killed.0) {
            gold.0 += definition.drop as usize;
        }
    }
}
