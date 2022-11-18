use bevy::prelude::*;
use bevy_egui::{egui::{self, Layout, Align, Ui, Button}, egui::panel::TopBottomSide, EguiContext, EguiPlugin};
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup_cubes)
        .add_system(ui_system)
        .run();
}

fn ui_system(mut egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx_mut();

    egui::TopBottomPanel::new(TopBottomSide::Bottom, "bottom_panel")
        .min_height(100.)
        .show(ctx, |ui| {
            let width = ui.available_width();
            let heigth = ui.available_height();
            let buttons_num = 3;
            let space = 100.;

            let button_width = (width - (space * (buttons_num + 1) as f32))/space;

            ui.with_layout(Layout::left_to_right(Align::Center).with_cross_justify(true), |ui| {
                ui.add_space(space);
                ui.add_sized([button_width, heigth], Button::new("Zasoby")); 
                ui.add_space(space);
                ui.add_sized([button_width, heigth], Button::new("Budynki")); 
                ui.add_space(space);
                ui.add_sized([button_width, heigth], Button::new("Jednostki")); 
                ui.add_space(space);
            });
        }); 
}

/// set up a simple 3D scene
fn setup_cubes(
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
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PickingCameraBundle::default(), // <- Sets the camera to use for picking.
    ));
}
