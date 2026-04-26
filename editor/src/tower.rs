use bevy_inspector_egui::reflect_inspector::ui_for_value;
use german_td_core::assets::towers::TowerAsset;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CreatedTowerDefinitions>();
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
struct CreatedTowerDefinitions {
    defs: Vec<Handle<TowerDefinition>>,
}

pub fn tower_tab_ui(world: &mut World, ui: &mut Ui) {
    ui.vertical(|ui| {
        if ui.button("Create new TowerDefinition").clicked() {
            let mut defs = world.resource_mut::<Assets<TowerDefinition>>();
            let handle = defs.add(TowerDefinition::default());
            world
                .resource_mut::<CreatedTowerDefinitions>()
                .defs
                .push(handle);
        }

        ui.separator();

        ui.label("TowerDefinitions");
        for handle in world
            .resource::<Assets<TowerDefinition>>()
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
        {
            let defs = world.resource::<Assets<TowerDefinition>>();
            ui.collapsing(defs.get(handle).unwrap().name.clone(), |ui| {
                tower_edit_ui(world, ui, handle);
            });
        }
    });
}

fn tower_edit_ui(world: &mut World, ui: &mut Ui, handle: AssetId<TowerDefinition>) {
    let type_registry = world.resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();

    let mut defs = world.resource_mut::<Assets<TowerDefinition>>();
    let tower_def = defs.get_mut(handle).unwrap();
    let def = &mut tower_def.asset;

    ui.horizontal(|ui| {
        ui.label("name:");
        ui.text_edit_singleline(&mut def.name);
    });

    ui.horizontal(|ui| {
        ui.label("gltf:");
        select_asset::<Gltf, TowerAsset, _, ()>(
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
        select_asset::<Image, TowerAsset, _, ()>(
            world,
            ui,
            handle,
            GetAssetNameFrom::Path,
            0,
            |asset, _| &mut asset.icon,
            None,
        );
    });
    ui.horizontal(|ui| {
        ui.label("projectile:");
        select_asset::<ProjectileDefinition, TowerAsset, _, ()>(
            world,
            ui,
            handle,
            GetAssetNameFrom::FileStem,
            0,
            |asset, _| &mut asset.projectile,
            None,
        );
    });
    let mut defs = world.resource_mut::<Assets<TowerDefinition>>();
    let tower_def = defs.get_mut(handle).unwrap();
    let def = &mut tower_def.asset;

    ui.horizontal(|ui| {
        ui.label("damage_factor:");
        egui::DragValue::new(&mut def.damage_factor).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("attack_duration_ms:");
        egui::DragValue::new(&mut def.attack_duration_ms).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("range:");
        egui::DragValue::new(&mut def.range).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("cost:");
        egui::DragValue::new(&mut def.cost).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("starter_tower:");
        ui.checkbox(&mut def.starter_tower, "buildable on plot");
    });

    ui.horizontal(|ui| {
        ui.label("damage_type:");
        ui_for_value(&mut def.damage_type, ui, &type_registry);
    });

    let mut create_upgrade = false;
    let len = def.upgrades.len();
    egui::Frame::new()
        .fill(egui::Color32::from_rgb(40, 40, 40))
        .corner_radius(5)
        .inner_margin(egui::Vec2::new(10., 5.))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("upgrades:");
                    ui.take_available_space();
                    if ui.button("Add").clicked() {
                        create_upgrade = true;
                    }
                });
                let mut remove_index = None;
                for i in 0..len {
                    if let Some(i) = select_asset::<TowerDefinition, TowerAsset, _, Option<usize>>(
                        world,
                        ui,
                        handle,
                        GetAssetNameFrom::T(Box::new(|t| t.asset.name.clone())),
                        i,
                        |asset, id| &mut asset.upgrades[id],
                        Some(&|stash, id| {
                            *stash = Some(id);
                        }),
                    ) {
                        remove_index.replace(i);
                    }
                }
                if let Some(index) = remove_index {
                    let mut defs = world.resource_mut::<Assets<TowerDefinition>>();
                    let tower_def = defs.get_mut(handle).unwrap();
                    let def = &mut tower_def.asset;
                    def.upgrades.remove(index);
                }
            });
        });

    let mut defs = world.resource_mut::<Assets<TowerDefinition>>();
    let tower_def = defs.get_mut(handle).unwrap();
    let def = &mut tower_def.asset;

    if create_upgrade {
        def.upgrades.push(String::new());
    }
}
