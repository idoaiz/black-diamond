mod components;
mod diamond;
mod dig;
#[cfg(feature = "debug-grid")]
mod grid;
mod map;
mod player;
mod systems;
mod game_timer;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // Plugins
    app.add_plugins(DefaultPlugins);

    // Resources
    app.insert_resource(game_timer::GameTimer::default());

    // Startup systems
    app.add_systems(
        Startup,
        (
            camera_setup,
            map::setup,
            player::setup,
            diamond::setup.after(map::setup),
            game_timer::setup_text
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
            game_timer::update_timer,
            game_timer::update_text,
        ),
    );

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
