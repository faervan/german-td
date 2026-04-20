use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_assets::<GenericIcons>();
}

#[derive(Asset, Resource, Reflect)]
#[reflect(Resource)]
pub struct GenericIcons {
    pub delete: Handle<Image>,
}

impl FromWorld for GenericIcons {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            delete: asset_server.load("icons/delete.png"),
        }
    }
}
