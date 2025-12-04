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
