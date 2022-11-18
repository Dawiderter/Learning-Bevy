use bevy::prelude::*;
use bevy_egui::{egui::{self, Layout, Align, Ui, Button}, egui::panel::TopBottomSide, EguiContext, EguiPlugin};
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::*;
use leafwing_input_manager::prelude::*;

use camera::CameraPlugin;
mod camera;

use board::BoardPlugin;
mod board;

use unit::UnitPlugin;
mod unit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(UnitPlugin)
        .run();
}
