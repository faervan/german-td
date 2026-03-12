use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<NotEnoughGold>();
    app.add_message::<EnemyKilled>();
}

#[derive(Debug, Default, Message, Reflect)]
pub struct NotEnoughGold;

#[derive(Debug, Message, Reflect)]
pub struct EnemyKilled(pub Handle<EnemyDefinition>);
