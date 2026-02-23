mod prelude;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (camera, demo))
        .run();
}

fn camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3 {
            x: 0.0,
            y: 100.0,
            z: 100.0,
        })
        .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn demo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)))),
        MeshMaterial3d(materials.add(Color::Srgba(Srgba {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }))),
    ));
}
