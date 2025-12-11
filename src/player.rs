use crate::components::{bounded::Bounded};
use bevy::prelude::*;
use serde::Deserialize;
use std::fs;
use crate::movement::{Speed, MoveDirection};

#[derive(Resource, TypePath, Deserialize, Debug, Clone)]
pub struct PlayerConfig {
    pub speed: f32,
    pub dig_cooldown: f32,
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

#[derive(Component, Default)]
pub enum PlayerState {
    Walking,
    #[default]
    Idle,
}

#[derive(Component, Default)]
pub enum FaceDirection {
    #[default]
    Down,
    Up,
    Left,
    Right,
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_config = PlayerConfig::load();
    commands.insert_resource(player_config.clone());

    commands.spawn((
        Player,
        PlayerState::default(),
        FaceDirection::default(),
        MoveDirection::default(),
        Speed(player_config.speed),
        Bounded::default(),
        Sprite {
            image: asset_server.load("player/player_standing.png"),
            custom_size: Some(Vec2::new(64., 64.)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut FaceDirection, &mut MoveDirection, &mut PlayerState), With<Player>>,
) {
    for (mut face_direction, mut move_direction, mut state) in query.iter_mut() {
        let mut input_direction = Vec3::ZERO;
        let mut input_facing = None;

        if keyboard.pressed(KeyCode::ArrowUp) {
            input_direction.y += 1.0;
            input_facing = Some(FaceDirection::Up);
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            input_direction.y -= 1.0;
            input_facing = Some(FaceDirection::Down);
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            input_direction.x -= 1.0;
            input_facing = Some(FaceDirection::Left);
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            input_direction.x += 1.0;
            input_facing = Some(FaceDirection::Right);
        }

        if let Some(facing) = input_facing {
            *face_direction = facing;
        }

        if input_direction.length_squared() > 0.0 {
            *move_direction = MoveDirection::new(input_direction.x, input_direction.y);
            *state = PlayerState::Walking;
        } else {
            *move_direction = MoveDirection::default();
            *state = PlayerState::Idle;
        }
    }
}