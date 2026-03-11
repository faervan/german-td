// Feels a bit unnecessary

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gold(500));
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Gold(usize);
