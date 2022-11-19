use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::scene::SceneBundle;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::*;
use leafwing_input_manager::prelude::*;

pub struct UnitPlugin;

#[derive(Component)]
struct Rzepa;

fn setup_unit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
    server: Res<AssetServer>,
) {
    let rzepa: Handle<Scene> = server.load("models/rzepa.gltf#Scene0");

    commands.spawn((
        SceneBundle {
            scene: rzepa,
            transform: Transform::from_xyz(0.0, 0.5, 0.0).with_rotation(Quat::from_rotation_x(-45.)),
            ..default()
        },
        PickableBundle::default(),
        meshes.add(Mesh::from(shape::Cube {size: 1.0})),
        Rzepa,
    ));
}

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_unit);
    }
}
