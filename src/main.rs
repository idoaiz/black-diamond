mod components;
mod diamond;
mod dig;
#[cfg(feature = "debug-grid")]
mod grid;
mod map;
mod player;
mod systems;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // Plugins
    app.add_plugins(DefaultPlugins);

    // Startup systems
    app.add_systems(
        Startup,
        (
            camera_setup,
            map::setup,
            player::setup,
            diamond::setup.after(map::setup),
        ),
    );

    #[cfg(feature = "debug-grid")]
    app.add_systems(Startup, grid::setup);

    // Update systems
    app.add_systems(
        Update,
        (
            player::move_player,
            dig::dig,
            dig::dig_cooldown,
            systems::clamp::clamp.after(player::move_player),
            systems::fade_out::fade_out,
        ),
    );

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
