use crate::prelude::*;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct GltfAnimationTarget(pub Entity);
