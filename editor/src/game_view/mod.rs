use bevy::input::mouse::AccumulatedMouseMotion;
use german_td_core::utils::{grab_cursor, ungrab_cursor};

use crate::prelude::*;

mod actor_preview;
mod map_preview;
pub use actor_preview::ActorPreview;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((actor_preview::plugin, map_preview::plugin));

    app.add_sub_state::<GameViewState>();

    app.add_systems(OnEnter(State::Editor), setup);
    app.add_systems(
        Update,
        (movement, rotation.run_if(input_pressed(MouseButton::Right)))
            .run_if(in_state(State::Editor)),
    );

    app.add_systems(
        Update,
        grab_cursor.run_if(input_just_pressed(MouseButton::Right)),
    );
    app.add_systems(
        Update,
        ungrab_cursor.run_if(input_just_released(MouseButton::Right)),
    );

    app.add_systems(
        OnEnter(GameViewState::MapPreview),
        switch_game_view_camera::<true>,
    );
    app.add_systems(
        OnEnter(GameViewState::ActorPreview),
        switch_game_view_camera::<false>,
    );
}

#[derive(Component)]
pub struct GameViewCamera;

#[derive(Component)]
struct MapPreviewCamera;

#[derive(Component)]
struct ActorPreviewCamera;

#[derive(SubStates, PartialEq, Eq, Debug, Default, Clone, Copy, Hash)]
#[source(State = State::Editor)]
pub(crate) enum GameViewState {
    #[default]
    MapPreview,
    ActorPreview,
}

fn setup(mut commands: Commands) {
    info!(
        "\nCamera controls:\n\
        WASD, Space/Shift+Space to move\n\
        Hold right click + move cursor to rotate\n\
        \n\
        Keybinds:\n\
        <LeftMouse> on entity to select\n\
        <S-LeftMouse> on entity to add to selection (keep previous selected)\n\
        <S-a> to select all\n\
        <S-Esc> to unselect all\n\
        <x> to despawn selected\n\
        <g> to move selection\n\
        <x>/<y>/<z> to restrict movement to axis (twice to local axis)\n\
        <Esc> to cancel selection movement\n\
        <LeftMouse> to confirm selection movement\n\
        <C-RightMouse> to move selected to cursor position\n\
        <f> to connect two selected waypoints or to mark waypoint as spawner\n\
        \n\
        <C-s> to save\n\
        <C-q> to save and quit\n\
        <C-a> to open the spawn menu\n\
        <Esc> to close the floating menus\n\
        <A-g> to toggle aabb gizmo"
    );

    commands.spawn((
        Name::new("GameViewCamera: Map Preview"),
        GameViewCamera,
        MapPreviewCamera,
        Camera3d::default(),
        MeshPickingCamera,
        Transform::from_xyz(0., 120., 120.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("GameViewCamera: Actor Preview"),
        ActorPreviewCamera,
        Camera3d::default(),
        MeshPickingCamera,
        Transform::from_xyz(0., 50., 2100.).looking_at(Vec3::new(0., 0., 2000.), Vec3::Y),
    ));
}

fn switch_game_view_camera<const SET_MAP_PREVIEW: bool>(
    mut commands: Commands,
    mut map_camera: Single<(Entity, &mut Camera), With<MapPreviewCamera>>,
    mut actor_camera: Single<
        (Entity, &mut Camera),
        (With<ActorPreviewCamera>, Without<MapPreviewCamera>),
    >,
) {
    if SET_MAP_PREVIEW {
        commands.entity(map_camera.0).insert(GameViewCamera);
        commands.entity(actor_camera.0).remove::<GameViewCamera>();
    } else {
        commands.entity(actor_camera.0).insert(GameViewCamera);
        commands.entity(map_camera.0).remove::<GameViewCamera>();
    }
    map_camera.1.is_active = SET_MAP_PREVIEW;
    actor_camera.1.is_active = !SET_MAP_PREVIEW;
}

const MOVE_SPEED: f32 = 50.;
fn movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<&mut Transform, With<GameViewCamera>>,
) {
    if input.pressed(KeyCode::ControlLeft)
        || input.pressed(KeyCode::ShiftLeft) && input.pressed(KeyCode::KeyA)
    {
        return;
    }

    for mut transform in query {
        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) {
            direction += Vec3::NEG_Z;
        }
        if input.pressed(KeyCode::KeyA) {
            direction += Vec3::NEG_X;
        }
        if input.pressed(KeyCode::KeyS) {
            direction += Vec3::Z;
        }
        if input.pressed(KeyCode::KeyD) {
            direction += Vec3::X;
        }
        if input.pressed(KeyCode::Space) {
            if input.pressed(KeyCode::ShiftLeft) {
                direction += Vec3::NEG_Y;
            } else {
                direction += Vec3::Y;
            }
        }

        let rotation = transform.rotation;
        transform.translation += rotation * direction * MOVE_SPEED * time.delta_secs();
    }
}

fn rotation(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<&mut Transform, With<GameViewCamera>>,
) {
    let Ok(mut camera) = query.single_mut() else {
        return;
    };

    let delta = mouse_motion.delta;
    if delta != Vec2::ZERO {
        let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);

        let yaw = yaw + delta.x * -0.001;
        let pitch = (pitch + delta.y * -0.001).clamp(-0.9, 0.9);

        camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
