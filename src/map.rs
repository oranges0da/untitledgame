use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bg)
            .add_systems(Startup, spawn_map);
    }
}

fn spawn_bg(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("map/bg.png"),
        ..default()
    });
}

fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    const TILE_SIZE: f32 = 12.7; // pixel size of single tile from tileset (not in-game size)
    const SCALE: f32 = 3.;
    const GROUND_LEVEL: f32 = -100.;

    let texture_handle = asset_server.load("map/tiles.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_SIZE, TILE_SIZE),
        10,
        10,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for x in -50..50 {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE * SCALE,
                    GROUND_LEVEL,
                    1.,
                ))
                .with_scale(Vec3::new(SCALE, SCALE, 0.)),
                ..default()
            })
            .insert(Collider::cuboid(TILE_SIZE / 2., TILE_SIZE / 2.));

        for y in -240..=-100 {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE * SCALE,
                    y as f32,
                    1.,
                ))
                .with_scale(Vec3::new(SCALE, SCALE, 0.)),
                ..default()
            });
        }
    }
}
