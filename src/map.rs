use bevy::prelude::*;

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
    const SCALE: f32 = 3.;
    const SCALED_TILE_SIZE: f32 = SCALE * TILE_SIZE;
    const MAP_SIZE: i32 = 20;

    // Spawn rock.
    let texture_handle = asset_server.load("map/rock_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        4,
        4,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 0.9),
            scale: Vec3::new(SCALE, SCALE, 0.),
            ..default()
        },
        sprite: TextureAtlasSprite {
            index: 0,
            ..default()
        },
        ..default()
    })
    .insert(GroundTile::Rock);

    // Spawn map.
    let texture_handle = asset_server.load("map/tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        11,
        20,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform {
                    scale: Vec3::new(SCALE, SCALE, 0.),
                    translation: Vec3::new(SCALED_TILE_SIZE * x as f32, SCALED_TILE_SIZE * y as f32, 0.8),
                    ..default()
                },
                sprite: TextureAtlasSprite {
                    index: 11,
                    ..default()
                },
                ..default()
            })
            .insert(GroundTile::Grass);
        }
    }
}
