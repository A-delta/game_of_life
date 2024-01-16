mod camera;
mod game_of_life;
mod game_of_life_logic;
mod ui;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use camera::CameraPlugin;
use game_of_life::GameOfLifePlugin;
use ui::UIPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins((GameOfLifePlugin, UIPlugin, CameraPlugin))
        .run();
}
