use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::*;
use leafwing_input_manager::prelude::*;

pub struct UnitPlugin;

fn setup_unit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
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

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_unit);
    }
}
