use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite::from_image(asset_server.load("player/player_standing.png")),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
