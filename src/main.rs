mod animation;
mod components;
mod diamond;
mod dig;
mod game_timer;
#[cfg(feature = "debug-grid")]
mod grid;
mod map;
mod movement;
mod player;
mod systems;

use crate::dig::DigEvent;
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

fn main() {
    let mut app = App::new();

    // Plugins
    app.add_plugins(DefaultPlugins);

    // States
    app.init_state::<GameState>();

    // Resources
    app.insert_resource(game_timer::GameTimer::default());

    // Events
    app.add_message::<DigEvent>();

    // Startup systems
    app.add_systems(
        Startup,
        (
            camera_setup,
            map::setup,
            player::setup,
            diamond::setup.after(map::setup),
            game_timer::setup_text,
        ),
    );

    #[cfg(feature = "debug-grid")]
    app.add_systems(Startup, grid::setup);

    // Update systems
    app.add_systems(
        Update,
        (
            player::handle_input,
            player::set_animation.after(player::handle_input),
            movement::move_players.after(player::set_animation),
            animation::animate,
            dig::dig,
            dig::dig_cooldown,
            diamond::detect_pickup,
            systems::clamp::clamp.after(movement::move_players),
            systems::fade_out::fade_out,
            game_timer::update_timer,
            game_timer::update_text,
            animation::animate,
        )
            .run_if(in_state(GameState::Playing)),
    );

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
