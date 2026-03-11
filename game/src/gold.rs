use bevy::color::palettes::css::GOLD;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gold(500));

    app.add_systems(OnEnter(AppState::Game), setup_gold_ui);

    app.add_systems(Update, update_gold_ui.run_if(in_state(AppState::Game)));
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Gold(usize);

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct GoldText;

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
        });
}

fn update_gold_ui(gold: Res<Gold>, mut gold_text: Single<&mut Text, With<GoldText>>) {
    gold_text.0 = format!("Gold: {}", gold.0);
}
