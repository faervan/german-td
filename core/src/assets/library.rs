use crate::{assets::AssetNameExt, prelude::*};

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct AssetLibrary<T: Asset> {
    pub entries: HashMap<String, Handle<T>>,
}

impl<T: Asset> Clone for AssetLibrary<T> {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
        }
    }
}

pub trait LibraryInitExt {
    fn init_library<T, STATE>(&mut self, on_exit: STATE) -> &mut Self
    where
        T: Asset + AssetNameExt + AssetLoadedHook,
        STATE: States;
}

impl LibraryInitExt for App {
    fn init_library<T, STATE>(&mut self, on_exit: STATE) -> &mut Self
    where
        T: Asset + AssetNameExt + AssetLoadedHook,
        STATE: States,
    {
        self.add_systems(OnExit(on_exit), |world: &mut World| {
            world.resource_scope(|world, mut assets: Mut<Assets<T>>| {
                let entries = assets
                    .iter_mut()
                    .map(|(id, asset)| {
                        asset.on_loaded_hook(world);
                        (id, asset.get_name())
                    })
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
                world.insert_resource(AssetLibrary { entries });
            });
        });
        self
    }
}

pub trait AssetLoadedHook {
    #[allow(unused)]
    /// The hook will be executed once the app leaves the loading state, right before inserting the
    /// [`AssetLibrary`]
    fn on_loaded_hook(&mut self, world: &mut World) {}
}
