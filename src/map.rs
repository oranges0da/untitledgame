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

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::new(SCALE, SCALE, 0.)),
        sprite: TextureAtlasSprite {
            index: 1,
            ..default()
        },
        ..default()
    })
    .insert(GroundTile::Grass);
}
