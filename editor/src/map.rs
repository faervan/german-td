use crate::prelude::*;

pub(super) fn plugin(_app: &mut App) {}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Waypoint;
