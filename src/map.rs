use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

#[derive(Resource, Deserialize, Debug, Clone)]
pub struct MapConfig {
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
}

impl MapConfig {
    pub fn load() -> Self {
        let contents =
            fs::read_to_string("assets/map/config.toml").expect("Failed reading map config file");
        toml::from_str(&contents).expect("Failed parsing map config file")
    }
}

// Tile types
#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Grass,
    Dirt,
}

fn generate_map(rows: usize, cols: usize) -> Vec<Vec<TileType>> {
    let mut map = vec![vec![TileType::Grass; rows]; cols];

    // Add dirt border
    for x in 0..rows {
        map[0][x] = TileType::Dirt;
        map[cols - 1][x] = TileType::Dirt;
    }
    for y in 0..cols {
        map[y][0] = TileType::Dirt;
        map[y][rows - 1] = TileType::Dirt;
    }

    map
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_config = MapConfig::load();
    commands.insert_resource(map_config.clone());

    let (width, height, tile_size) = (
        map_config.width as f32,
        map_config.height as f32,
        map_config.tile_size as f32,
    );
    let map = generate_map(
        (width / tile_size).ceil() as usize,
        (height / tile_size).ceil() as usize,
    );

    // Spawn tiles
    for (y, row) in map.iter().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
            let texture_path = get_texture(*tile_type);

            // Calculate the tile starting coordinate (bottom-left)
            let x_pos = (x as f32) * tile_size - width / 2.0;
            let y_pos = (y as f32) * tile_size - height / 2.0;

            commands.spawn((
                Sprite::from_image(asset_server.load(texture_path)),
                // Define the transform as the center of the tile, so the bottom left is (x_pos, y_pos)
                Transform::from_xyz(x_pos + tile_size / 2.0, y_pos + tile_size / 2.0, 0.0),
            ));
        }
    }
}

fn get_texture(tile_type: TileType) -> &'static str {
    match tile_type {
        TileType::Grass => "map/grass.png",
        TileType::Dirt => "map/dirt.png",
    }
}
