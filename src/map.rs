use bevy::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader};

#[derive(Component)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(PreStartup, spawn_map);
    }
}

#[derive(Component)]
pub enum GroundTile {
    Grass,
    Dirt,
    Rock,
}

// Spawn tile blocks.
fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const TILE_SIZE: f32 = 16.; // Size of single tile in tileset.
    const SCALE: f32 = 4.;
    const SCALED_TILE_SIZE: f32 = SCALE * TILE_SIZE;
    const MAP_SIZE: i32 = 20;

    // Cut out rock sprite.
    let texture_handle = asset_server.load("map/rock_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        4,
        4,
        None,
        None,
    );
    let rock_texture = texture_atlases.add(texture_atlas);

    // Cut out grass tile.
    let texture_handle = asset_server.load("map/tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        11,
        20,
        None,
        None,
    );
    let tile_texture = texture_atlases.add(texture_atlas);

    // Open level file.
    let mut file = File::open("assets/map/level.txt").unwrap();
    let reader = BufReader::new(file);

    let mut x: f32 = 0.;
    let mut y: f32 = 0.;

    for line in reader.lines() {
        y += 1.;
        x = 0.;
        for char in line.expect("Failed to get line for map.").chars() {
            x += 1.;
            commands.spawn(SpriteSheetBundle {
                texture_atlas: tile_texture.clone(),
                sprite: TextureAtlasSprite {
                    index: 122,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(SCALED_TILE_SIZE * x, SCALED_TILE_SIZE * y, 0.),
                    scale: Vec3::new(SCALE, SCALE, 0.),
                    ..default()
                },
                ..default()
            });

            if char == '1' {
                commands.spawn(SpriteSheetBundle {
                    texture_atlas: rock_texture.clone(),
                    sprite: TextureAtlasSprite {
                        index: 0,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(SCALED_TILE_SIZE * x, SCALED_TILE_SIZE * y, 0.9),
                        scale: Vec3::new(SCALE, SCALE, 0.),
                        ..default()
                    },
                    ..default()
                });
            }
        }
    }
}
