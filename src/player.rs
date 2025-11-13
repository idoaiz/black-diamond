use std::fs;
use bevy::prelude::*;
use serde::Deserialize;


#[derive(Resource, TypePath, Deserialize, Debug, Clone)]
pub struct PlayerConfig {
    pub speed: f32,
}

impl PlayerConfig {
    pub fn load() -> Self {
        let contents = fs::read_to_string("assets/player/config.toml")
            .expect("Failed reading player config file");
        toml::from_str(&contents)
            .expect("Failed parsing player config file")
    }
}

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_config = PlayerConfig::load();
    commands.insert_resource(player_config);

    commands.spawn((
        Player,
        Sprite::from_image(asset_server.load("player/player_standing.png")),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}
