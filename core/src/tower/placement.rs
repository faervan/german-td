use bevy::{
    pbr::decal::{ForwardDecal, ForwardDecalMaterial, ForwardDecalMaterialExt},
    render::render_resource::AsBindGroup,
};

use crate::prelude::*;

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_plugins(MaterialPlugin::<ForwardDecalMaterial<TowerPlotMaterial>>::default());

        app.add_message::<SpawnTowerPlacement>();

        app.add_systems(
            Update,
            spawn_placements.run_if(on_message::<SpawnTowerPlacement>.and(in_state(game_state))),
        );
    }
}

#[derive(Message, Debug)]
pub struct SpawnTowerPlacement {
    pub position: Vec3,
}

#[derive(Component)]
struct UnassignedTowerPlot;

fn spawn_placements(
    mut events: MessageReader<SpawnTowerPlacement>,
    mut commands: Commands,
    mut decal_standard_materials: ResMut<Assets<ForwardDecalMaterial<TowerPlotMaterial>>>,
) {
    for spawn in events.read() {
        commands.spawn((
            Name::new("Tower plot"),
            UnassignedTowerPlot,
            ForwardDecal,
            MeshMaterial3d(decal_standard_materials.add(ForwardDecalMaterial {
                base: TowerPlotMaterial::default(),
                extension: ForwardDecalMaterialExt {
                    depth_fade_factor: 10.,
                },
            })),
            Transform::from_translation(spawn.position).with_scale(Vec3::splat(10.)),
            // Use physics picking, as there is no mesh picking for decals
            PhysicsPickable,
            RigidBody::Static,
            Collider::cylinder(0.5, 0.1)
        )).observe(|
            event: On<Pointer<Over>>,
            query: Query<&MeshMaterial3d<ForwardDecalMaterial<TowerPlotMaterial>>>,
            mut materials: ResMut<Assets<ForwardDecalMaterial<TowerPlotMaterial>>>| {
                if let Ok(handle) = query.get(event.entity)
                    && let Some(material) = materials.get_mut(&handle.0) {
                        material.base.hover = 1.;
                }
        }).observe(|
            event: On<Pointer<Out>>,
            query: Query<&MeshMaterial3d<ForwardDecalMaterial<TowerPlotMaterial>>>,
            mut materials: ResMut<Assets<ForwardDecalMaterial<TowerPlotMaterial>>>| {
                if let Ok(handle) = query.get(event.entity)
                    && let Some(material) = materials.get_mut(&handle.0) {
                        material.base.hover = 0.;
                }
        }).observe(|
            event: On<Pointer<Click>>,
            query: Query<&Transform>,
            mut commands: Commands,
            mut tower_spawner: MessageWriter<SpawnTower>,
            tower_lib: TowerLibrary| {
                commands.entity(event.entity).despawn();
                if let Ok(transform) = query.get(event.entity) {
                    tower_spawner.write(SpawnTower {
                        position: transform.translation,
                        definition: tower_lib.entries["Bow Turret"].clone()
                    });
                }
        });
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
struct TowerPlotMaterial {
    #[uniform(0)]
    /// 0 means not hovered, 1 means hovered
    hover: f32,
}

impl Material for TowerPlotMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/tower_plot.wgsl".into()
    }
    fn deferred_fragment_shader() -> bevy::shader::ShaderRef {
        "shaders/tower_plot.wgsl".into()
    }
}
