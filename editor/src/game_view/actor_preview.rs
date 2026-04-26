use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(State::Editor), setup);

    app.init_state::<ActorPreview>();
    app.add_systems(
        Update,
        change_actor.run_if(resource_changed::<bevy::prelude::State<ActorPreview>>),
    );
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActorPreview {
    #[default]
    None,
    Tower(Handle<TowerDefinition>),
    Projectile(Handle<ProjectileDefinition>),
    Enemy(Handle<EnemyDefinition>),
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        Name::new("Actor Preview Ground"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(200.)))),
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE))),
        Transform::from_xyz(0., 0., 2000.),
        NotShadowCaster,
    ));

    commands.spawn((
        PointLight {
            range: 500.,
            intensity: 1_000_000_000.,
            shadows_enabled: true,
            color: Color::WHITE,
            ..Default::default()
        },
        Transform::from_xyz(0., 100., 1900.),
    ));
}

#[derive(Component)]
struct Actor;

fn change_actor(
    state: Res<bevy::prelude::State<ActorPreview>>,
    mut commands: Commands,
    actors: Query<Entity, With<Actor>>,
    mut tower_spawner: MessageWriter<SpawnTower>,
    mut projectile_spawner: MessageWriter<SpawnProjectile>,
    mut enemy_spawner: MessageWriter<SpawnEnemy>,
) {
    for entity in actors {
        commands.entity(entity).despawn();
    }
    let pos = Vec3::new(0., 1., 2000.);
    match state.get() {
        ActorPreview::None => {}
        ActorPreview::Tower(handle) => {
            tower_spawner.write(SpawnTower {
                position: pos,
                definition: handle.clone(),
                despawn_entities: vec![],
            });
        }
        ActorPreview::Projectile(handle) => {
            projectile_spawner.write(SpawnProjectile {
                position: pos,
                definition: handle.clone(),
                damage_factor: 1.,
                damage_type: DamageType::Area {
                    radius: 10.,
                    target_pos: pos + Vec3::Y * 100.,
                },
            });
        }
        ActorPreview::Enemy(handle) => {
            enemy_spawner.write(SpawnEnemy {
                position: pos,
                definition: handle.clone(),
                waypoints: vec![].into(),
            });
        }
    }
}
