use super::consts::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use bevy::prelude::*;

// Tile types
#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Grass,
    Dirt,
}

fn generate_map() -> Vec<Vec<TileType>> {
    let mut map = vec![vec![TileType::Grass; MAP_WIDTH]; MAP_HEIGHT];

    // Add dirt border
    for x in 0..MAP_WIDTH {
        map[0][x] = TileType::Dirt;
        map[MAP_HEIGHT - 1][x] = TileType::Dirt;
    }
    for y in 0..MAP_HEIGHT {
        map[y][0] = TileType::Dirt;
        map[y][MAP_WIDTH - 1] = TileType::Dirt;
    }

    map
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = generate_map();

    // Spawn tiles
    for (y, row) in map.iter().enumerate() {
        for (x, tile_type) in row.iter().enumerate() {
            let texture_path = get_texture(*tile_type);

            // Calculate position (centered on screen)
            let x_pos = (x as f32 - MAP_WIDTH as f32 / 2.0) * TILE_SIZE as f32;
            let y_pos = (MAP_HEIGHT as f32 / 2.0 - y as f32) * TILE_SIZE as f32;

            commands.spawn((
                Sprite::from_image(asset_server.load(texture_path)),
                Transform::from_xyz(x_pos, y_pos, 0.0),
            ));
        }
    }
}

fn get_texture(tile_type: TileType) -> &'static str {
    match tile_type {
        TileType::Grass => "tiles/grass.png",
        TileType::Dirt => "tiles/dirt.png",
    }
}
