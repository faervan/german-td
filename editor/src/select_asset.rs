use std::ops::DerefMut;

use german_td_core::assets::RonAsset;

use crate::prelude::*;

/// Create a text input modifying a string value of an asset definition specified by the provided
/// `def_access`.
/// The bevy assets of type `Target` are queried and the user can select each one to fill out the
/// string buffer with its name.
/// How the name is obtained can be specified by [`GetAssetNameFrom`].
/// `RonA` is our plain [`RonAsset`], e.g. [`TowerAsset`]
/// The actual [`bevy::Asset`] is given by [`RonAsset::Asset`], for [`TowerAsset`] this is
/// [`TowerDefinition`].
pub fn select_asset<Target, RonA, F, STASH>(
    world: &mut World,
    ui: &mut Ui,
    handle: AssetId<RonA::Asset>,
    name_getter: GetAssetNameFrom<Target>,
    id: usize,
    def_access: F,
    remove_callback: Option<&dyn Fn(&mut STASH, usize)>,
) -> STASH
where
    Target: Asset,
    RonA: RonAsset,
    RonA::Asset: DerefMut<Target = RonA>,
    F: Fn(&mut RonA, usize) -> &mut String,
    STASH: Default,
{
    let mut stash = STASH::default();

    ui.vertical(|ui| {
        let asset_server = world.resource::<AssetServer>();
        let assets = world
            .resource::<Assets<Target>>()
            .iter()
            .filter_map(|(id, t)| match &name_getter {
                GetAssetNameFrom::Path => asset_server
                    .get_path(id)
                    .and_then(|p| p.path().to_str().map(ToString::to_string)),
                GetAssetNameFrom::FileStem => asset_server.get_path(id).and_then(|p| {
                    p.path()
                        .file_stem()
                        .and_then(|p| p.to_str().map(ToString::to_string))
                }),
                GetAssetNameFrom::T(f) => Some(f(t)),
            })
            .collect::<Vec<_>>();

        let mut defs = world.resource_mut::<Assets<RonA::Asset>>();
        let tower_def = defs.get_mut(handle).unwrap();
        let def = &mut tower_def.deref_mut();

        let mut show_suggestions = false;

        ui.horizontal(|ui| {
            let edit = ui.text_edit_singleline(def_access(def, id));
            if edit.has_focus() || edit.lost_focus() {
                show_suggestions = true;
            }
            if let Some(callback) = remove_callback
                && ui.button("Remove").clicked()
            {
                callback(&mut stash, id);
            }
        });
        if show_suggestions {
            for asset in assets {
                if ui.button(&asset).clicked() {
                    *def_access(def, id) = asset;
                }
            }
        }
    });

    stash
}

pub enum GetAssetNameFrom<A> {
    Path,
    FileStem,
    T(Box<dyn Fn(&A) -> String>),
}
