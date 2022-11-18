use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::*;
use leafwing_input_manager::prelude::*;

#[derive(Component)]
struct PlayerCamera;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Move,
}

const CAMERA_SPEED: f32 = 5.0;

fn player_movement_system(
    mut query: Query<(&mut Transform, &ActionState<Action>), With<PlayerCamera>>,
    time: Res<Time>,
) {
    let (mut transform, action_state) = query.single_mut();
    if action_state.pressed(Action::Move) {
        let axis_pair = action_state.axis_pair(Action::Move).unwrap();
        transform.translation +=
            Vec3::new(axis_pair.x(), 0.0, -axis_pair.y()) * time.delta_seconds() * CAMERA_SPEED;
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PickingCameraBundle::default(), // <- Sets the camera to use for picking.
        PlayerCamera,
        InputManagerBundle::<Action> {
            // Stores "which actions are currently activated"
            // Map some arbitrary keys into a virtual direction pad that triggers our move action
            input_map: InputMap::new([(
                VirtualDPad {
                    up: KeyCode::W.into(),
                    down: KeyCode::S.into(),
                    left: KeyCode::A.into(),
                    right: KeyCode::D.into(),
                },
                Action::Move,
            )])
            .build(),
            ..default()
        },
    ));
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        PickableBundle::default(),
    ));
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        PickableBundle::default(),
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(spawn_camera)
        .add_system(player_movement_system)
        .add_startup_system(setup)
        .run();
}
