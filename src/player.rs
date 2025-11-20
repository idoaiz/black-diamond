use crate::components::{bounded::Bounded, speed::Speed};
use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

#[derive(Resource, TypePath, Deserialize, Debug, Clone)]
pub struct PlayerConfig {
    pub speed: f32,
}

impl PlayerConfig {
    pub fn load() -> Self {
        let contents = fs::read_to_string("assets/player/config.toml")
            .expect("Failed reading player config file");
        toml::from_str(&contents).expect("Failed parsing player config file")
    }
}

#[derive(Component)]
pub struct Player;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_config = PlayerConfig::load();
    commands.insert_resource(player_config.clone());

    commands.spawn((
        Player,
        Speed(player_config.speed),
        Bounded::default(),
        Sprite {
            image: asset_server.load("player/player_standing.png"),
            custom_size: Some(Vec2::new(64.,64.)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

pub fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let (mut transform, speed) = query.single_mut().expect("More than one player!");
    let dt = time.delta_secs();
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction.length_squared() > 0.0 {
        transform.translation += direction.normalize() * speed.0 * dt;
    }
}
