use bevy_inspector_egui::reflect_inspector::ui_for_value;
use german_td_core::assets::enemies::EnemyAsset;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CreatedEnemyDefinitions>();
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
struct CreatedEnemyDefinitions {
    defs: Vec<Handle<EnemyDefinition>>,
}

pub fn enemy_tab_ui(world: &mut World, ui: &mut Ui) {
    ui.vertical(|ui| {
        if ui.button("Create new EnemyDefinition").clicked() {
            let mut defs = world.resource_mut::<Assets<EnemyDefinition>>();
            let handle = defs.add(EnemyDefinition::default());
            world
                .resource_mut::<CreatedEnemyDefinitions>()
                .defs
                .push(handle);
        }

        ui.separator();

        ui.label("EnemyDefinitions");
        for handle in world
            .resource::<Assets<EnemyDefinition>>()
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
        {
            let defs = world.resource::<Assets<EnemyDefinition>>();
            ui.collapsing(defs.get(handle).unwrap().name.clone(), |ui| {
                enemy_edit_ui(world, ui, handle);
            });
        }
    });
}

fn enemy_edit_ui(world: &mut World, ui: &mut Ui, handle: AssetId<EnemyDefinition>) {
    let type_registry = world.resource::<AppTypeRegistry>().0.clone();
    let type_registry = type_registry.read();

    if ui.button("Preview").clicked() {
        let mut defs = world.resource_mut::<Assets<EnemyDefinition>>();
        let handle = defs.get_strong_handle(handle).unwrap();
        world
            .resource_mut::<NextState<ActorPreview>>()
            .set(ActorPreview::Enemy(handle));
    }

    let mut defs = world.resource_mut::<Assets<EnemyDefinition>>();
    let enemy_def = defs.get_mut(handle).unwrap();
    let def = &mut enemy_def.asset;

    ui.horizontal(|ui| {
        ui.label("name:");
        ui.text_edit_singleline(&mut def.name);
    });

    ui.horizontal(|ui| {
        ui.label("gltf:");
        select_asset::<Gltf, EnemyAsset, _, ()>(
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
        select_asset::<Image, EnemyAsset, _, ()>(
            world,
            ui,
            handle,
            GetAssetNameFrom::Path,
            0,
            |asset, _| &mut asset.icon,
            None,
        );
    });

    let mut defs = world.resource_mut::<Assets<EnemyDefinition>>();
    let enemy_def = defs.get_mut(handle).unwrap();
    let def = &mut enemy_def.asset;

    ui.horizontal(|ui| {
        ui.label("damage:");
        egui::DragValue::new(&mut def.damage).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("attack_duration_ms:");
        egui::DragValue::new(&mut def.attack_duration_ms).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("walk_speed:");
        egui::DragValue::new(&mut def.walk_speed).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("health:");
        egui::DragValue::new(&mut def.health).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("drop:");
        egui::DragValue::new(&mut def.drop).ui(ui);
    });

    ui.horizontal(|ui| {
        ui.label("idle_animation:");
        ui.push_id(0, |ui| {
            ui_for_value(&mut def.idle_animation, ui, &type_registry);
        });
    });

    ui.horizontal(|ui| {
        ui.label("walk_animation:");
        ui.push_id(1, |ui| {
            ui_for_value(&mut def.walk_animation, ui, &type_registry);
        });
    });

    ui.horizontal(|ui| {
        ui.label("attack_animation:");
        ui.push_id(2, |ui| {
            ui_for_value(&mut def.attack_animation, ui, &type_registry);
        });
    });
}
