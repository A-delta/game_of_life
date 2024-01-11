mod camera;
mod game_of_life;
mod game_of_life_logic;
use bevy::prelude::*;
use camera::CameraPlugin;
use game_of_life::GameOfLifePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GameOfLifePlugin, CameraPlugin))
        .run();
}
