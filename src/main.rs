mod consts;
mod map;
mod player;
mod window;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window::plugin()))
        .add_systems(Startup, (camera_setup, map::setup, player::setup))
        .run();
}

fn camera_setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);
}
