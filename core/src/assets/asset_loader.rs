use crate::prelude::*;

use bevy::asset::{AssetLoader, AsyncReadExt};
use serde::de::DeserializeOwned;

pub trait AssetNameExt {
    fn get_name(&self) -> String;
}

pub trait RonAsset: TypePath + DeserializeOwned + Send + Sync + 'static {
    type Asset: Asset;
    const DIRECTORY: &str;
    const EXTENSION: &str;

    fn path(name: &str) -> PathBuf {
        PathBuf::from_iter([
            PathBuf::from(Self::DIRECTORY),
            PathBuf::from(format!("{name}.{}", Self::EXTENSION)),
        ])
    }

    fn load_dependencies(
        self,
        context: &mut bevy::asset::LoadContext<'_>,
    ) -> impl Future<Output = Self::Asset> + Send + Sync;
}

#[derive(TypePath)]
pub struct RonAssetLoader<T> {
    _phantom: PhantomData<T>,
}

impl<T> Default for RonAssetLoader<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T: RonAsset> AssetLoader for RonAssetLoader<T> {
    type Asset = T::Asset;
    type Settings = ();
    type Error = RonAssetLoadError;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>>
    {
        async {
            let mut bytes = String::new();
            reader.read_to_string(&mut bytes).await?;
            let t: T = ron::from_str(&bytes)?;
            Ok(T::load_dependencies(t, load_context).await)
        }
    }

    fn extensions(&self) -> &[&str] {
        &[T::EXTENSION]
    }
}

#[derive(Error, Debug)]
pub enum RonAssetLoadError {
    #[error("Asset reader failed: {0}")]
    AssetReaderFailed(#[from] bevy::tasks::futures_lite::io::Error),
    #[error("Asset deserialization failed: {0}")]
    DeserializationError(#[from] ron::error::SpannedError),
}
