use super::consts::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    for x in 0..=MAP_WIDTH {
        let x_pos = (x as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE as f32;
        commands.spawn((vertical_gridline(), Transform::from_xyz(x_pos, 0.0, 1.0)));
    }

    for y in 0..=MAP_HEIGHT {
        let y_pos = (MAP_HEIGHT as f32 - y as f32 / 2.0) * TILE_SIZE as f32;
        commands.spawn((horizontal_gridline(), Transform::from_xyz(0.0, y_pos, 1.0)));
    }
}

fn vertical_gridline() -> Sprite {
    Sprite {
        color: Color::srgba(1., 1., 1., 0.3),
        custom_size: Some(Vec2::new(1., MAP_HEIGHT as f32)),
        ..default()
    }
}

fn horizontal_gridline() -> Sprite {
    Sprite {
        color: Color::srgba(1., 1., 1., 0.3),
        custom_size: Some(Vec2::new(MAP_WIDTH as f32, 1.)),
        ..default()
    }
}
