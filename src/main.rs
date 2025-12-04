mod grid;
mod map;
mod player;
mod components;
mod systems;
mod black_diamond;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (camera_setup, map::setup, player::setup, grid::setup),
        )
        .add_systems(
            Startup,
            black_diamond::setup.after(map::setup)
        )
        .add_systems(
            Update,
            (
                player::move_player,
                systems::clamp::clamp.after(player::move_player),
            ),
        )
        .run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
