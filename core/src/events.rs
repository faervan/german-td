use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<NotEnoughGold>();
}

#[derive(Debug, Default, Message, Reflect)]
pub struct NotEnoughGold;
