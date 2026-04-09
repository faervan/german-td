use bevy_inspector_egui::bevy_inspector::ui_for_value;

use crate::{focus::EntitySelectChange, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<SpawnWaypoint>();
    app.add_message::<SpawnTowerPlot>();
    app.add_message::<SpawnEnemyPaths>();

    app.init_resource::<EnemyPaths>();

    app.add_systems(
        Update,
        (
            spawn_waypoints,
            spawn_plots,
            spawn_paths.after(spawn_waypoints),
        )
            .run_if(in_state(State::Editor)),
    );
    app.add_systems(Update, draw_paths.run_if(in_state(State::Editor)));
    app.add_systems(
        Update,
        add_path_connection.run_if(in_state(State::Editor).and(input_just_pressed(KeyCode::KeyF))),
    );
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

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource, Default)]
pub struct EnemyPaths {
    /// The currently edited path index, if any
    editing: Option<usize>,
    paths: Vec<EnemyPath>,
}

#[derive(Reflect, Debug, Default)]
#[reflect(Default)]
struct EnemyPath {
    /// Connections between two waypoint entities
    connections: Vec<(Entity, Entity)>,
    spawner: Option<(Entity, Handle<ScriptAsset>)>,
}

#[derive(Message)]
pub struct SpawnEnemyPaths {
    pub map_definition: Handle<MapDefinition>,
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

fn spawn_paths(
    mut paths: ResMut<EnemyPaths>,
    mut events: MessageReader<SpawnEnemyPaths>,
    map_defs: Res<Assets<MapDefinition>>,
    waypoints: Query<(&Transform, Entity), With<Waypoint>>,
) {
    for event in events.read() {
        if let Some(definition) = map_defs.get(&event.map_definition) {
            paths.paths = definition
                .paths
                .iter()
                .map(|path| {
                    let mut connections = vec![];
                    let mut last_entity = None;
                    for waypoint_index in &path.waypoints {
                        let Some(new_entity) = waypoints.iter().find_map(|(transform, entity)| {
                            (transform.translation == definition.waypoints()[*waypoint_index])
                                .then_some(entity)
                        }) else {
                            continue;
                        };
                        if let Some(entity) = last_entity.take() {
                            connections.push((entity, new_entity));
                        }
                        last_entity = Some(new_entity);
                    }
                    EnemyPath {
                        spawner: connections.first().map(|(e, _)| (*e, path.spawner.clone())),
                        connections,
                    }
                })
                .collect();
        }
    }
}

fn draw_paths(
    mut gizmos: Gizmos,
    paths: Res<EnemyPaths>,
    query: Query<&Transform, With<Waypoint>>,
) {
    // Unfortunate naming here
    for (path_index, path) in paths.paths.iter().enumerate() {
        if let Some((entity, _)) = &path.spawner
            && let Ok(transform) = query.get(*entity)
        {
            gizmos.cube(
                transform.with_scale(Vec3::splat(5.)),
                match paths.editing == Some(path_index) {
                    true => Color::srgb(1., 0., 1.),
                    false => Color::srgba(1., 0., 1., 0.5),
                },
            );
        }
        for connection in &path.connections {
            if let Ok(w1) = query.get(connection.0)
                && let Ok(w2) = query.get(connection.1)
            {
                let color = match paths.editing {
                    Some(index) if index == path_index => Color::srgb(1., 0., 1.),
                    Some(_) => Color::srgba(1., 0., 1., 0.5),
                    None => Color::srgba(1., 0., 1., 0.8),
                };
                gizmos.line(w1.translation, w2.translation, color);
            }
        }
    }
}

fn add_path_connection(
    mut paths: ResMut<EnemyPaths>,
    focused: Res<FocusedEntities>,
    mut scripts: ResMut<Assets<ScriptAsset>>,
    query: Query<(), With<Waypoint>>,
) {
    let Some(path_index) = paths.editing else {
        warn!("No path selected!");
        return;
    };
    let all_are_waypoints = focused.entities.iter().all(|id| query.contains(*id));
    if focused.entities.len() == 1 && all_are_waypoints {
        if paths.paths[path_index]
            .spawner
            .take()
            .is_none_or(|(entity, _)| entity != focused.entities[0])
        {
            let runtime = scripting::enemy_spawner_runtime();
            let script = match ScriptAsset::new(
                &runtime,
                PathBuf::from_iter(["scripts", "spawners", "NewSpawner.roto"])
                    .display()
                    .to_string(),
                String::new(),
            ) {
                Ok(script) => script,
                Err(e) => {
                    error!("Failed to compile roto test script for spawner: {e}");
                    return;
                }
            };
            paths.paths[path_index].spawner = Some((focused.entities[0], scripts.add(script)));
        }
    } else if focused.entities.len() == 2 && all_are_waypoints {
        paths.paths[path_index]
            .connections
            .push((focused.entities[0], focused.entities[1]));
    } else {
        warn!("Exactly one or two waypoints need to be selected");
    }
}

pub fn path_edit_ui(world: &mut World, ui: &mut Ui) {
    world.resource_scope(|world, mut paths: Mut<EnemyPaths>| {
        match paths.editing {
            Some(index) => {
                ui.label(format!("Editing path: {index}"));
                if ui.button("Clear editing").clicked() {
                    paths.editing = None;
                }
            }
            None => {
                ui.label("Editing path: None");
            }
        }
        if ui.button("Add new path").clicked() {
            paths.paths.push(Default::default());
        }
        let mut editing = None;
        let mut delete = None;
        for (index, path) in paths.paths.iter_mut().enumerate() {
            ui.separator();
            ui.collapsing(format!("Path {index}:"), |ui| {
                if ui.button("Select for editing").clicked() {
                    editing = Some(index);
                }
                if ui.button("Delete path").clicked() {
                    delete = Some(index);
                }
                ui.horizontal(|ui| {
                    ui.label("connections");
                    ui_for_value(&mut path.connections, ui, world);
                });
                ui.horizontal(|ui| {
                    ui.label("spawner");
                    match &mut path.spawner {
                        Some((entity, handle)) => {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("entity");
                                    ui_for_value(entity, ui, world);
                                });
                                let mut scripts = world.resource_mut::<Assets<ScriptAsset>>();
                                let script = scripts.get_mut(handle).unwrap();
                                ui.horizontal(|ui| {
                                    let file_path = &mut script.file;
                                    ui.label("file_path");
                                    ui.text_edit_singleline(file_path);
                                });
                                ui.label("Script:");
                                ui.take_available_width();
                                ui.add(
                                    egui::TextEdit::multiline(&mut script.source)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        }
                        None => {
                            ui.label("None");
                        }
                    }
                });
            });
        }
        if editing.is_some() {
            paths.editing = editing;
        }
        if let Some(index) = delete.take() {
            if let Some(edit_index) = paths.editing {
                if edit_index == index {
                    paths.editing = None;
                } else if edit_index > index {
                    paths.editing = Some(edit_index - 1);
                }
            }
            paths.paths.remove(index);
        }
    });
}

pub fn save(
    mut definitions: ResMut<Assets<MapDefinition>>,
    map: Query<&Map>,
    waypoints: Query<&Transform, With<Waypoint>>,
    plots: Query<&Transform, With<TowerPlot>>,
    paths: Res<EnemyPaths>,
) {
    let Ok(map) = map.single() else {
        return;
    };
    let Some(definition) = definitions.get_mut(&map.definition) else {
        warn!("Failed to save map data: no MapDefinition found for handle");
        return;
    };
    definition.set_waypoints(
        waypoints
            .iter()
            .map(|transform| transform.translation)
            .collect(),
    );
    definition.tower_plots = plots
        .iter()
        .map(|transform| transform.translation)
        .collect();
    definition.paths = paths
        .paths
        .iter()
        .filter_map(|path| {
            let mut path_waypoints = vec![];
            let mut used_waypoint_entities = vec![];
            let (spawner_entity, spawner) = path.spawner.clone()?;
            let mut next_entity = Some(spawner_entity);
            while let Some(entity) = next_entity {
                if let Ok(transform) = waypoints.get(entity) {
                    path_waypoints.push(
                        definition
                            .waypoints()
                            .iter()
                            .position(|pos| *pos == transform.translation)
                            .unwrap(),
                    );
                    next_entity = path.connections.iter().find_map(|(x, y)| {
                        (*x == entity && !used_waypoint_entities.contains(y))
                            .then_some(y)
                            .or_else(|| {
                                (*y == entity && !used_waypoint_entities.contains(x)).then_some(x)
                            })
                            .copied()
                    });
                    used_waypoint_entities.push(entity);
                }
            }
            Some(crate::prelude::assets::maps::EnemyPath {
                waypoints: path_waypoints,
                spawner,
            })
        })
        .collect();
}
