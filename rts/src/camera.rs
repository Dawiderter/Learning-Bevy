use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
struct PlayerCamera;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum MoveAction {
    Move,
}
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum ZoomAction {
    In,
    Out,
}

const CAMERA_SPEED: f32 = 5.0;

pub struct CameraPlugin;

fn player_movement_system(
    mut query: Query<
        (
            &mut Transform,
            &ActionState<MoveAction>,
            &ActionState<ZoomAction>,
        ),
        With<PlayerCamera>,
    >,
    time: Res<Time>,
) {
    let (mut transform, move_action, zoom_action) = query.single_mut();
    let mut vec = Vec3::new(0.0, 0.0, 0.0);
    if move_action.pressed(MoveAction::Move) {
        let axis_pair = move_action.axis_pair(MoveAction::Move).unwrap();
        vec += Vec3::new(axis_pair.x(), 0.0, -axis_pair.y());
    }
    if zoom_action.pressed(ZoomAction::In) {
        vec += Vec3::new(0.0, -2.0, 0.0);
    }
    if zoom_action.pressed(ZoomAction::Out) {
        vec += Vec3::new(0.0, 2.0, 0.0);
    }
    transform.translation += vec * time.delta_seconds() * CAMERA_SPEED;
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PickingCameraBundle::default(), // <- Sets the camera to use for picking.
        PlayerCamera,
        InputManagerBundle::<MoveAction> {
            // Stores "which actions are currently activated"
            // Map some arbitrary keys into a virtual direction pad that triggers our move action
            input_map: InputMap::new([(
                VirtualDPad {
                    up: KeyCode::W.into(),
                    down: KeyCode::S.into(),
                    left: KeyCode::A.into(),
                    right: KeyCode::D.into(),
                },
                MoveAction::Move,
            )])
            .build(),
            ..default()
        },
        InputManagerBundle::<ZoomAction> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::Equals, ZoomAction::In),
                (KeyCode::Minus, ZoomAction::Out),
            ]),
        },
    ));
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_plugin(InputManagerPlugin::<MoveAction>::default())
            .add_plugin(InputManagerPlugin::<ZoomAction>::default())
            .add_system(player_movement_system);
    }
}
