use bevy::color::palettes::css::{GOLD, RED};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gold(40));

    app.add_systems(OnEnter(AppState::Game), setup_gold_ui);

    app.add_systems(Update, update_gold_ui.run_if(in_state(AppState::Game)));

    app.add_systems(
        Update,
        update_not_enough_gold_ui.run_if(on_message::<NotEnoughGold>.and(in_state(AppState::Game))),
    );
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct GoldText;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct NotEnoughGoldText;

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
                NotEnoughGoldText,
            ));
        });
}

fn update_gold_ui(gold: Res<Gold>, mut gold_text: Single<&mut Text, With<GoldText>>) {
    gold_text.0 = format!("Gold: {}", gold.0);
}

fn update_not_enough_gold_ui(
    mut not_enough_gold: MessageReader<NotEnoughGold>,
    mut not_enough_gold_text: Single<&mut Text, With<NotEnoughGoldText>>,
) {
    for _not_enough_gold in not_enough_gold.read() {
        not_enough_gold_text.0 = format!("NOT ENOUGH GOLD");
    }
}
