use crate::{focus::EntitySelectChange, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<SpawnWaypoint>();
    app.add_message::<SpawnTowerPlot>();

    app.add_systems(Update, spawn_waypoints.run_if(in_state(State::Editor)));
    app.add_systems(Update, spawn_plots.run_if(in_state(State::Editor)));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Waypoint;

#[derive(Message)]
pub struct SpawnWaypoint {
    pub position: Option<Vec3>,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TowerPlot;

#[derive(Message)]
pub struct SpawnTowerPlot {
    pub position: Option<Vec3>,
}

fn spawn_waypoints(
    mut commands: Commands,
    mut events: MessageReader<SpawnWaypoint>,
    transform: Single<&Transform, With<EditorCursor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for spawn in events.read() {
        let position = spawn.position.unwrap_or(transform.translation);
        commands
            .spawn((
                Name::new("Waypoint"),
                Waypoint,
                FocusableEntity,
                Transform::from_translation(position),
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
    mut events: MessageReader<SpawnTowerPlot>,
    transform: Single<&Transform, With<EditorCursor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for spawn in events.read() {
        let position = spawn.position.unwrap_or(transform.translation);
        commands
            .spawn((
                Name::new("TowerPlot"),
                TowerPlot,
                FocusableEntity,
                Transform::from_translation(position)
                    .with_rotation(Quat::from_rotation_x(-PI / 2.)),
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

pub fn save(
    mut definitions: ResMut<Assets<MapDefinition>>,
    map: Query<&Map>,
    waypoints: Query<&Transform, With<Waypoint>>,
    plots: Query<&Transform, With<TowerPlot>>,
) {
    let Ok(map) = map.single() else {
        return;
    };
    let Some(definition) = definitions.get_mut(&map.definition) else {
        warn!("Failed to save map data: no MapDefinition found for handle");
        return;
    };
    definition.waypoints = waypoints
        .iter()
        .map(|transform| transform.translation)
        .collect();
    definition.tower_plots = plots
        .iter()
        .map(|transform| transform.translation)
        .collect();
}
