pub use std::collections::{HashMap, HashSet, VecDeque};
pub use std::f32::consts::{PI, TAU};
pub use std::marker::PhantomData;
pub use std::path::PathBuf;
pub use std::sync::Arc;
pub use std::time::Duration;

pub use serde::{Deserialize, Serialize};
pub use thiserror::Error;

pub use avian3d::prelude::*;
pub use bevy::asset::ReflectAsset;
pub use bevy::ecs::{lifecycle::HookContext, world::DeferredWorld};
pub use bevy::input::common_conditions::{input_just_pressed, input_just_released, input_pressed};
pub use bevy::light::{NotShadowCaster, NotShadowReceiver};
pub use bevy::prelude::*;

pub type EnemyLibrary<'a> = Res<'a, AssetLibrary<EnemyDefinition>>;
pub type MapLibrary<'a> = Res<'a, AssetLibrary<MapDefinition>>;
pub type TowerLibrary<'a> = Res<'a, AssetLibrary<TowerDefinition>>;
pub type ProjectileLibrary<'a> = Res<'a, AssetLibrary<ProjectileDefinition>>;

pub use crate::assets;
pub use crate::assets::AssetLibrary;
pub use crate::assets::AssetResourceLoader as _;
pub use crate::assets::LibraryInitExt as _;
pub use crate::assets::all_assets_loaded;
pub use crate::assets::audio::GameSoundHandles;
pub use crate::assets::enemies::EnemyDefinition;
pub use crate::assets::maps::MapDefinition;
pub use crate::assets::projectile::ProjectileDefinition;
pub use crate::assets::roto_asset::ScriptAsset;
pub use crate::assets::towers::TowerDefinition;

pub use crate::physics_layers::GameLayer;
pub use crate::scripting::ScriptAssetExt as _;
pub use crate::utils::InspectNoneExt as _;
pub use crate::utils::audio::*;
pub use crate::utils::billboard::*;
pub use crate::utils::delayed_despawn::*;
pub use crate::utils::value_animation::AnimateValueExt as _;

pub use crate::components::*;
pub use crate::enemy::*;
pub use crate::maps::*;
pub use crate::projectile::*;
pub use crate::scripting;
pub use crate::tower::*;

#[cfg(feature = "dev_native")]
pub use bevy_egui::PrimaryEguiContext;
#[cfg(feature = "dev_native")]
pub use bevy_inspector_egui::{bevy_egui::EguiPlugin, prelude::*, quick::WorldInspectorPlugin};
