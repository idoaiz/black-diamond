use bevy::prelude::*;

const GRID_SPACING_X: u32 = 64;
const GRID_SPACING_Y: u32 = 64;

pub fn setup(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single().expect("Couldn't get window");
    let width = window.width();
    let height = window.height();

    let mut x = 0.;
    while x <= (width / 2.) {
        commands.spawn((vertical_gridline(height), Transform::from_xyz(x, 0.0, 1.0)));
        commands.spawn((vertical_gridline(height), Transform::from_xyz(-x, 0.0, 1.0)));
        x += GRID_SPACING_X as f32;
    }

    let mut y = 0.;
    while y <= (height / 2.) {
        commands.spawn((horizontal_gridline(width), Transform::from_xyz(0.0, y, 1.0)));
        commands.spawn((
            horizontal_gridline(width),
            Transform::from_xyz(0.0, -y, 1.0),
        ));
        y += GRID_SPACING_Y as f32;
    }
}

fn vertical_gridline(height: f32) -> Sprite {
    Sprite {
        color: Color::srgba(1., 1., 1., 0.3),
        custom_size: Some(Vec2::new(1., height)),
        ..default()
    }
}

fn horizontal_gridline(width: f32) -> Sprite {
    Sprite {
        color: Color::srgba(1., 1., 1., 0.3),
        custom_size: Some(Vec2::new(width, 1.)),
        ..default()
    }
}
