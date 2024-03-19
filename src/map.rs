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

    // Cut out certain tile from tileset.
    let texture_handle = asset_server.load("map/rock_tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE + 2.),
        4,
        4,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                scale: Vec3::new(SCALE, SCALE, 0.),
                ..default()
            },
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            ..default()
        })
        .insert(GroundTile::Grass);
}
