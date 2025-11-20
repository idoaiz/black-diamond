use bevy::prelude::*;

const GRID_SPACING_X: u32 = 64;
const GRID_SPACING_Y: u32 = 64;

pub fn setup(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single().expect("Couldn't get window");
    let width = window.width();
    let height = window.height();

    for x in (0..=width as usize).step_by(GRID_SPACING_X as usize) {
        commands.spawn((
            vertical_gridline(height),
            Transform::from_xyz((x as f32) - width / 2.0, 0.0, 1.0),
        ));
    }

    for y in (0..=height as usize).step_by(GRID_SPACING_Y as usize) {
        commands.spawn((
            horizontal_gridline(width),
            Transform::from_xyz(0.0, (y as f32) - height / 2.0, 1.0),
        ));
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
