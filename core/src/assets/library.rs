use crate::{assets::AssetNameExt, prelude::*};

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct AssetLibrary<T: Asset> {
    pub entries: HashMap<String, Handle<T>>,
}

pub trait LibraryInitExt {
    fn init_library<T: Asset + AssetNameExt, STATE: States>(&mut self, on_exit: STATE)
    -> &mut Self;
}

impl LibraryInitExt for App {
    fn init_library<T: Asset + AssetNameExt, STATE: States>(
        &mut self,
        on_exit: STATE,
    ) -> &mut Self {
        self.add_systems(
            OnExit(on_exit),
            |mut commands: Commands, mut assets: ResMut<Assets<T>>| {
                let entries = assets
                    .iter()
                    .map(|(id, asset)| (id, asset.get_name()))
                    .collect::<Vec<(AssetId<T>, String)>>()
                    .into_iter()
                    .filter_map(|(id, name)| match assets.get_strong_handle(id) {
                        Some(handle) => Some((name, handle)),
                        None => {
                            warn!("Failed to get strong handle for {id}");
                            None
                        }
                    })
                    .collect();
                commands.insert_resource(AssetLibrary { entries });
            },
        );
        self
    }
}
