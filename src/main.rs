mod grid;
mod map;
mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (camera_setup, map::setup, player::setup, grid::setup),
        )
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
