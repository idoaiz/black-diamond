use crate::animation::Animation;
use crate::components::bounded::Bounded;
use crate::movement::{MoveDirection, Speed};
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerGeneralConfig {
    pub speed: f32,
    pub dig_cooldown: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PlayerAnimationConfig {
    #[serde(flatten)]
    pub animations: HashMap<String, Vec<usize>>,
}

#[derive(Resource, TypePath, Deserialize, Debug, Clone)]
pub struct PlayerConfig {
    pub general: PlayerGeneralConfig,
    pub animation: PlayerAnimationConfig,
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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load the player config file and set it up as resource
    let player_config = PlayerConfig::load();
    commands.insert_resource(player_config.clone());

    // Load atlas
    let spritesheet = asset_server.load("spritesheets/sokoban.png");
    let initial_animation = player_config.animation.animations.get("idle_down").unwrap();
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 13, 8, None, None);
    let atlas_layout = atlas_layouts.add(layout);
    let atlas = TextureAtlas {
        layout: atlas_layout.clone(),
        index: initial_animation[0],
    };

    commands.spawn((
        Player,
        PlayerState::default(),
        FaceDirection::default(),
        MoveDirection::default(),
        Speed(player_config.general.speed),
        Bounded::default(),
        Sprite {
            image: spritesheet.clone(),
            texture_atlas: Some(atlas),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Animation::new(initial_animation.clone(), 0.2),
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

pub fn set_animation(
    mut query: Query<
        (&PlayerState, &FaceDirection, &mut Animation),
        (
            With<Player>,
            Or<(Changed<PlayerState>, Changed<FaceDirection>)>,
        ),
    >,
    player_config: Res<PlayerConfig>,
) {
    for (state, direction, mut animation) in query.iter_mut() {
        let state = match state {
            PlayerState::Idle => "idle",
            PlayerState::Walking => "walk",
        };
        let direction = match direction {
            FaceDirection::Up => "up",
            FaceDirection::Down => "down",
            FaceDirection::Left => "left",
            FaceDirection::Right => "right",
        };
        let key = format!("{}_{}", state, direction);
        if let Some(frames) = player_config.animation.animations.get(&key) {
            animation.frames = frames.clone();
        }
    }
}
