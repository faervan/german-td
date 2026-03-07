pub use roto::Val;
use roto::{List, NoCtx, Runtime, TypedFunc, library};

use crate::prelude::*;

pub fn enemy_spawner_runtime() -> Runtime<NoCtx> {
    let library = library! {
        #[clone] type EnemyLibrary = Val<AssetLibrary<EnemyDefinition>>;
        #[clone] type EnemyDefinitionHandle = Val<Handle<EnemyDefinition>>;
        #[copy] type Duration = Val<Duration>;
        #[clone] type SpawnItem = Val<(Duration, Handle<EnemyDefinition>)>;

        fn get_enemy_handle(name: Arc<str>, library: Val<AssetLibrary<EnemyDefinition>>) -> Option<Val<Handle<EnemyDefinition>>> {
            library.entries.get(name.as_str()).map(|h| Val(h.clone()))
        }

        fn range_f32(start: u32, end: u32) -> List<f32> {
            let range = List::new();
            for i in start..end {
                range.push(i as f32);
            }
            range
        }

        fn as_f32(n: u32) -> f32 {
            n as f32
        }

        fn error(msg: Arc<str>) {
            error!("{msg}")
        }

        impl Val<Duration> {
            fn from_secs(seconds: f32) -> Self {
                Val(Duration::from_secs_f32(seconds))
            }
        }

        impl Val<(Duration, Handle<EnemyDefinition>)> {
            fn new(duration: Val<Duration>, handle: Val<Handle<EnemyDefinition>>) -> Self {
                Val((duration.0, handle.0))
            }
        }
    };
    Runtime::from_lib(library).unwrap()
}

type SpawnerFunction =
    fn(u32, Val<AssetLibrary<EnemyDefinition>>) -> List<Val<(Duration, Handle<EnemyDefinition>)>>;

pub trait ScriptAssetExt {
    fn get_spawner_function(
        &self,
        assets: &mut Assets<ScriptAsset>,
    ) -> Option<TypedFunc<NoCtx, SpawnerFunction>>;
}

impl ScriptAssetExt for Handle<ScriptAsset> {
    fn get_spawner_function(
        &self,
        assets: &mut Assets<ScriptAsset>,
    ) -> Option<TypedFunc<NoCtx, SpawnerFunction>> {
        let script = assets.get_mut(self)?;
        script
            .package
            .get_function("spawn_wave")
            .inspect_err(|e| {
                error!(
                    "Failed to extract function spawn_wave from {}: {e}",
                    script.file
                )
            })
            .ok()
    }
}
