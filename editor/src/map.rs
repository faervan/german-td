use crate::{focus::EntitySelectChange, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<Waypoint>();
    app.add_message::<TowerPlot>();

    app.add_systems(Update, spawn_waypoints.run_if(in_state(State::Editor)));
    app.add_systems(Update, spawn_plots.run_if(in_state(State::Editor)));
}

#[derive(Component, Message, Reflect)]
#[reflect(Component)]
pub struct Waypoint;

#[derive(Component, Message, Reflect)]
#[reflect(Component)]
pub struct TowerPlot;

fn spawn_waypoints(
    mut commands: Commands,
    mut events: MessageReader<Waypoint>,
    transform: Single<&Transform, With<EditorCursor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in events.read() {
        commands
            .spawn((
                Name::new("Waypoint"),
                Waypoint,
                FocusableEntity,
                **transform,
                Mesh3d(meshes.add(Sphere::new(2.))),
                MeshMaterial3d(
                    materials.add(StandardMaterial::from_color(Color::srgba(1., 0., 0., 0.7))),
                ),
                NotShadowCaster,
                NotShadowReceiver,
            ))
            .observe(
                |event: On<EntitySelectChange>,
                 mut query: Query<&mut MeshMaterial3d<StandardMaterial>, With<Waypoint>>,
                 mut materials: ResMut<Assets<StandardMaterial>>| {
                    let Ok(mut material) = query.get_mut(event.target) else {
                        return;
                    };
                    let color = match event.selected {
                        true => Color::srgb(1., 0., 0.),
                        false => Color::srgba(1., 0., 0., 0.7),
                    };
                    material.0 = materials.add(StandardMaterial::from_color(color));
                },
            );
    }
}

fn spawn_plots(
    mut commands: Commands,
    mut events: MessageReader<TowerPlot>,
    transform: Single<&Transform, With<EditorCursor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in events.read() {
        commands
            .spawn((
                Name::new("TowerPlot"),
                TowerPlot,
                FocusableEntity,
                transform.with_rotation(Quat::from_rotation_x(-PI / 2.)),
                Mesh3d(meshes.add(Circle::new(3.))),
                MeshMaterial3d(
                    materials.add(StandardMaterial::from_color(Color::srgba(1., 0., 0., 0.7))),
                ),
                NotShadowCaster,
                NotShadowReceiver,
            ))
            .observe(
                |event: On<EntitySelectChange>,
                 mut query: Query<&mut MeshMaterial3d<StandardMaterial>, With<TowerPlot>>,
                 mut materials: ResMut<Assets<StandardMaterial>>| {
                    let Ok(mut material) = query.get_mut(event.target) else {
                        return;
                    };
                    let color = match event.selected {
                        true => Color::srgb(1., 0., 0.),
                        false => Color::srgba(1., 0., 0., 0.7),
                    };
                    material.0 = materials.add(StandardMaterial::from_color(color));
                },
            );
    }
}
