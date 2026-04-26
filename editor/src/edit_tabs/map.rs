use german_td_core::assets::maps::MapAsset;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CreatedMapDefinitions>();
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
struct CreatedMapDefinitions {
    defs: Vec<Handle<MapDefinition>>,
}

pub fn map_tab_ui(world: &mut World, ui: &mut Ui) {
    ui.vertical(|ui| {
        if ui.button("Create new MapDefinition").clicked() {
            let mut defs = world.resource_mut::<Assets<MapDefinition>>();
            let handle = defs.add(MapDefinition::default());
            world
                .resource_mut::<CreatedMapDefinitions>()
                .defs
                .push(handle);
        }

        ui.separator();

        ui.label("MapDefinitions");
        for handle in world
            .resource::<Assets<MapDefinition>>()
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
        {
            let defs = world.resource::<Assets<MapDefinition>>();
            ui.collapsing(defs.get(handle).unwrap().name.clone(), |ui| {
                map_edit_ui(world, ui, handle);
            });
        }
    });
}

fn map_edit_ui(world: &mut World, ui: &mut Ui, handle: AssetId<MapDefinition>) {
    let mut defs = world.resource_mut::<Assets<MapDefinition>>();
    let map_def = defs.get_mut(handle).unwrap();
    let def = &mut map_def.asset;

    ui.horizontal(|ui| {
        ui.label("name:");
        ui.text_edit_singleline(&mut def.name);
    });

    ui.horizontal(|ui| {
        ui.label("gltf:");
        select_asset::<Gltf, MapAsset, _, ()>(
            world,
            ui,
            handle,
            GetAssetNameFrom::Path,
            0,
            |asset, _| &mut asset.gltf,
            None,
        );
    });
    ui.horizontal(|ui| {
        ui.label("icon:");
        select_asset::<Image, MapAsset, _, ()>(
            world,
            ui,
            handle,
            GetAssetNameFrom::Path,
            0,
            |asset, _| &mut asset.icon,
            None,
        );
    });

    let mut defs = world.resource_mut::<Assets<MapDefinition>>();
    let map_def = defs.get_mut(handle).unwrap();
    let def = &mut map_def.asset;

    ui.horizontal(|ui| {
        ui.label("waves:");
        egui::DragValue::new(&mut def.waves).ui(ui);
    });
}
