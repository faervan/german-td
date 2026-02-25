pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::f32::consts::{PI, TAU};
pub use std::marker::PhantomData;
pub use std::time::Duration;

pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;

pub use avian3d::prelude::*;
pub use bevy::asset::ReflectAsset;
pub use bevy::prelude::*;

pub type EnemyLibrary<'a> =
    Res<'a, crate::assets::AssetLibrary<crate::assets::enemies::EnemyDefinition>>;
pub type TowerLibrary<'a> =
    Res<'a, crate::assets::AssetLibrary<crate::assets::towers::TowerDefinition>>;

pub use crate::assets::AssetResourceLoader as _;
pub use crate::assets::LibraryInitExt as _;
pub use crate::assets::all_assets_loaded;
pub use crate::assets::enemies::EnemyDefinition;
pub use crate::assets::towers::TowerDefinition;

pub use crate::enemy::*;
