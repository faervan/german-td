pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::f32::consts::{PI, TAU};
pub use std::marker::PhantomData;
pub use std::time::Duration;

pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;

pub use avian3d::prelude::*;
pub use bevy::asset::ReflectAsset;
pub use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
pub use bevy::prelude::*;

pub type EnemyLibrary<'a> = Res<'a, crate::assets::AssetLibrary<EnemyDefinition>>;
pub type MapLibrary<'a> = Res<'a, crate::assets::AssetLibrary<MapDefinition>>;
pub type TowerLibrary<'a> = Res<'a, crate::assets::AssetLibrary<TowerDefinition>>;
pub type ProjectileLibrary<'a> = Res<'a, crate::assets::AssetLibrary<ProjectileDefinition>>;

pub use crate::assets::AssetResourceLoader as _;
pub use crate::assets::LibraryInitExt as _;
pub use crate::assets::all_assets_loaded;
pub use crate::assets::enemies::EnemyDefinition;
pub use crate::assets::maps::MapDefinition;
pub use crate::assets::projectile::ProjectileDefinition;
pub use crate::assets::towers::TowerDefinition;
pub use crate::utils::InspectNoneExt as _;
pub use crate::utils::delayed_despawn::*;

pub use crate::components::*;
pub use crate::enemy::*;
pub use crate::maps::*;
pub use crate::projectile::*;
pub use crate::tower::*;

#[cfg(feature = "dev_native")]
pub use bevy_egui::PrimaryEguiContext;
#[cfg(feature = "dev_native")]
pub use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*, quick::WorldInspectorPlugin};
