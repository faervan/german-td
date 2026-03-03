use bevy::asset::{AssetLoader, AsyncReadExt};
use roto::{FileTree, NoCtx, Package, RotoReport, Runtime};

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_asset::<ScriptAsset>();
    app.register_asset_loader(ScriptAssetLoader);
}

#[derive(Asset, TypePath)]
pub struct ScriptAsset {
    pub file: String,
    pub source: String,
    pub package: Package<NoCtx>,
}

impl ScriptAsset {
    pub fn new(runtime: &Runtime<NoCtx>, file: String, source: String) -> Result<Self, RotoReport> {
        Ok(Self {
            package: FileTree::test_file(&file, &source, 0).compile(runtime)?,
            source,
            file,
        })
    }
}

#[derive(TypePath)]
struct ScriptAssetLoader;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScriptAssetLoaderSettings {
    pub file: String,
    #[serde(skip)]
    pub runtime: Runtime<NoCtx>,
}

impl AssetLoader for ScriptAssetLoader {
    type Asset = ScriptAsset;
    type Settings = ScriptAssetLoaderSettings;
    type Error = ScriptLoadError;

    fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        settings: &Self::Settings,
        _load_context: &mut bevy::asset::LoadContext,
    ) -> impl bevy::tasks::ConditionalSendFuture<Output = std::result::Result<Self::Asset, Self::Error>>
    {
        async {
            let mut source = String::new();
            reader.read_to_string(&mut source).await?;
            Ok(ScriptAsset::new(
                &settings.runtime,
                settings.file.clone(),
                source,
            )?)
        }
    }

    fn extensions(&self) -> &[&str] {
        &["roto"]
    }
}

#[derive(Error, Debug)]
enum ScriptLoadError {
    #[error("Reading the asset failed: {0}")]
    AssetReaderFailed(#[from] std::io::Error),
    #[error("An roto error occurred: {0}")]
    RotoError(#[from] RotoReport),
}
