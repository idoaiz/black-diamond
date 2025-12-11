use crate::GameState;
use crate::dig::DigEvent;
use crate::game_timer::GameTimer;
use crate::map::MapConfig;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Diamond;

pub fn setup(
    mut commands: Commands,
    map_config: Res<MapConfig>,
    #[cfg(feature = "debug-diamond-location")] asset_server: Res<AssetServer>,
) {
    let (x_min, x_max) = (
        -(map_config.width as f32 / 2.),
        (map_config.width as f32) / 2.,
    );
    let (y_min, y_max) = (
        -(map_config.height as f32 / 2.),
        (map_config.height as f32) / 2.,
    );
    let mut rng = rand::rng();
    let x = rng.random_range(x_min..x_max);
    let y = rng.random_range(y_min..y_max);
    commands.spawn((
        Diamond,
        Transform::from_xyz(x, y, 1.0),
        #[cfg(feature = "debug-diamond-location")]
        Sprite::from_image(asset_server.load("items/diamond.png")),
    ));
}

pub fn detect_pickup(
    mut dig_message_listener: MessageReader<DigEvent>,
    diamond_query: Query<&Transform, With<Diamond>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_timer: ResMut<GameTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let diamond_location = diamond_query.single().unwrap().translation;
    for dig_event in dig_message_listener.read() {
        if dig_event.location.distance(diamond_location) <= 50. {
            let diamond_found_title = (
                Text::new("You Found the Diamond!"),
                TextFont {
                    font: asset_server.load("fonts/KarmaticArcade.ttf"),
                    font_size: 60.0,
                    ..default()
                },
            );
            let timer_sub_title = (
                Text::new(format!("It took you {:.2} seconds", game_timer.time)),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
            );

            commands
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(10.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(diamond_found_title);
                    parent.spawn(timer_sub_title);
                });

            next_state.set(GameState::GameOver);
        }
    }
}
