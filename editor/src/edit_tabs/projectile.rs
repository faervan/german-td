use german_td_core::assets::projectile::ProjectileAsset;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CreatedProjectileDefinitions>();
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
struct CreatedProjectileDefinitions {
    defs: Vec<Handle<ProjectileDefinition>>,
}

pub fn projectile_tab_ui(world: &mut World, ui: &mut Ui) {
    ui.vertical(|ui| {
        if ui.button("Create new ProjectileDefinition").clicked() {
            let mut defs = world.resource_mut::<Assets<ProjectileDefinition>>();
            let handle = defs.add(ProjectileDefinition::default());
            world
                .resource_mut::<CreatedProjectileDefinitions>()
                .defs
                .push(handle);
        }

        ui.separator();

        ui.label("ProjectileDefinitions");
        for handle in world
            .resource::<Assets<ProjectileDefinition>>()
            .iter()
            .map(|(id, _)| id)
            .collect::<Vec<_>>()
        {
            let defs = world.resource::<Assets<ProjectileDefinition>>();
            ui.collapsing(defs.get(handle).unwrap().name.clone(), |ui| {
                projectile_edit_ui(world, ui, handle);
            });
        }
    });
}

fn projectile_edit_ui(world: &mut World, ui: &mut Ui, handle: AssetId<ProjectileDefinition>) {
    if ui.button("Preview").clicked() {
        let mut defs = world.resource_mut::<Assets<ProjectileDefinition>>();
        let handle = defs.get_strong_handle(handle).unwrap();
        world
            .resource_mut::<NextState<ActorPreview>>()
            .set(ActorPreview::Projectile(handle));
    }

    let mut defs = world.resource_mut::<Assets<ProjectileDefinition>>();
    let projectile_def = defs.get_mut(handle).unwrap();
    let def = &mut projectile_def.asset;

    ui.horizontal(|ui| {
        ui.label("name:");
        ui.text_edit_singleline(&mut def.name);
    });

    ui.horizontal(|ui| {
        ui.label("gltf:");
        select_asset::<Gltf, ProjectileAsset, _, ()>(
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
        select_asset::<Image, ProjectileAsset, _, ()>(
            world,
            ui,
            handle,
            GetAssetNameFrom::Path,
            0,
            |asset, _| &mut asset.icon,
            None,
        );
    });

    let mut defs = world.resource_mut::<Assets<ProjectileDefinition>>();
    let projectile_def = defs.get_mut(handle).unwrap();
    let def = &mut projectile_def.asset;

    ui.horizontal(|ui| {
        ui.label("damage:");
        egui::DragValue::new(&mut def.damage).ui(ui);
    });
}
