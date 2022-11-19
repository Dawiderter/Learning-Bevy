use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_mod_picking::*;
use leafwing_input_manager::prelude::*;

use camera::CameraPlugin;
use ui::UiPlugin;
mod camera;
mod ui;

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
        .add_plugin(UiPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(UnitPlugin)
        .run();
}
