use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::*;
use leafwing_input_manager::prelude::*;

use camera::CameraPlugin;
use ui::UiPlugin;
mod camera;
mod ui;

/// set up a simple 3D scene
fn setup_cubes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    ambient_light.color = Color::WHITE;
    ambient_light.brightness = 1.0;
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
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
    // camera
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(UiPlugin)
        .add_startup_system(setup_cubes)
        .run();
}
