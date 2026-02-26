use crate::{prelude::*, utils::on_ready_insert_animation_target};

pub(super) fn plugin<STATE: States + Copy>(game_state: STATE) -> impl Plugin {
    move |app: &mut App| {
        app.add_message::<SpawnTower>();

        app.add_systems(
            Update,
            (
                spawn_towers.run_if(on_message::<SpawnTower>),
                search_tower_target,
                attack_tower_target,
            )
                .run_if(in_state(game_state)),
        );
    }
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Tower {
    target: Option<Entity>,
    attack_timer: Timer,
}

#[derive(Message, Debug)]
pub struct SpawnTower {
    pub position: Vec3,
    pub definition: Handle<TowerDefinition>,
}

fn spawn_towers(
    mut events: MessageReader<SpawnTower>,
    mut commands: Commands,
    definitions: Res<Assets<TowerDefinition>>,
) {
    for spawn in events.read() {
        let def = definitions.get(&spawn.definition).unwrap();
        info!("Spawning tower {} at {:?}", def.name, spawn.position);

        let mut attack_timer = Timer::new(def.attack_duration, TimerMode::Repeating);
        attack_timer.finish();
        commands
            .spawn((
                Name::new(format!("Tower: {}", def.name)),
                Transform::from_translation(spawn.position),
                SceneRoot(def.scene.clone()),
                Tower {
                    target: None,
                    attack_timer,
                },
            ))
            .observe(on_ready_insert_animation_target);
    }
}

// TODO: This can probably be moved into collision event hooks?
// Sets the target of the Tower Component
fn search_tower_target(towers: Query<&mut Tower>, enemies: Query<Entity, With<Enemy>>) {
    for mut tower in towers {
        if tower.target.is_none() {
            tower.target = enemies.iter().next();
        }
    }
}

// TODO: Move out spawning of projectile
fn attack_tower_target(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut towers: Query<(&mut Tower, &Transform)>,
) {
    for (mut tower, transform) in &mut towers {
        if tower.attack_timer.is_finished()
            && let Some(target) = tower.target
        {
            commands.spawn((
                Projectile::new(target),
                Mesh3d(meshes.add(Cuboid::from_length(3.0))),
                MeshMaterial3d(materials.add(Color::Srgba(Srgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.0,
                    alpha: 1.0,
                }))),
                *transform,
                // Physics
                RigidBody::Kinematic,
                Collider::cylinder(0.3, 1.5),
                GravityScale(0.),
            ));
        }

        // tick timer
        tower.attack_timer.tick(time.delta());
    }
}
