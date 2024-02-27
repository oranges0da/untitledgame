use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
            app.add_systems(Startup, spawn_map);
    }
}

#[derive(Component)]
pub enum GroundTile {
    Grass,
    Dirt,
}

// Try to spawn single tile for now.
fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const TILE_SIZE: f32 = 16.; // Size of single tile in tileset.
    const SCALE: f32 = 3.;
    const SCALED_TILE_SIZE: f32 = SCALE * TILE_SIZE;
    const MAP_SIZE: i32 = 20;

    // Cut out certain tile from tileset.
    let texture_handle = asset_server.load("map/tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE + 2.),
        11,
        10,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Generate row of blocks.
    for x in 0..MAP_SIZE {
        let x_offset: f32 = (x as f32 * SCALED_TILE_SIZE * 0.5);
        let y_offset: f32 = (x as f32 * SCALED_TILE_SIZE * 0.25);

        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(SCALED_TILE_SIZE + x_offset, -SCALED_TILE_SIZE - y_offset, 0.),
                scale: Vec3::new(SCALE, SCALE, 0.),
                ..default()
            },
            sprite: TextureAtlasSprite {
                index: 1,
                ..default()
            },
            ..default()
        })
        .insert(GroundTile::Grass);
    }
}
