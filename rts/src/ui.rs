use bevy::prelude::*;
use bevy_egui::{egui::{self, Layout, Align, Ui, Button, Grid}, egui::panel::TopBottomSide, EguiContext, EguiPlugin};

const PADDING : f32 = 50.;

fn ui_system(mut egui_context: ResMut<EguiContext>) {
    let ctx = egui_context.ctx_mut();

    egui::TopBottomPanel::new(TopBottomSide::Bottom, "bottom_panel")
        .min_height(100.)
        .show(ctx, |ui| {
            Grid::new("grid").spacing((50.,50.)).show(ui, |ui| {
                ui.button("Budynki");
                ui.button("Jednostki");
                ui.button("Zasoby");
            });
        }); 
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(ui_system);
    }
}
