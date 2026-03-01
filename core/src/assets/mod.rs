use crate::prelude::*;

mod asset_loader;
pub use asset_loader::*;

mod resource_loader;
pub use resource_loader::*;

mod library;
pub use library::*;

pub mod enemies;
pub mod maps;
pub mod projectile;
pub mod towers;

pub(super) fn plugin<STATE: States + Copy>(loading_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_plugins((
            resource_loader::plugin,
            enemies::plugin(loading_state),
            maps::plugin(loading_state),
            towers::plugin(loading_state),
            projectile::plugin(loading_state),
        ));
    }
}
