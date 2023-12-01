use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map)
            .add_systems(Update, spawn_bg);
    }
}

fn spawn_bg(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn earth in background as it would appear on the moon.
    commands.spawn(SpriteBundle {
        texture: asset_server.load("map/earth.png"),
        transform: Transform {
            translation: Vec3::new(0., 0., -1.),
            scale: Vec3::new(8., 8., 0.),
            ..default()
        }
        .with_scale(Vec3::new(8., 8., 0.)),
        ..default()
    });

    // Spawn "star" (not working right now).
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(shape::Circle::new(1.5).into()).into(),
    //     material: materials.add(ColorMaterial::from(Color::WHITE)),
    //     transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
    //     ..default()
    // });
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

    // Spawn ground with tiles from spritesheet.
    for x in -50..50 {
        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x as f32 * TILE_SIZE * SCALE, GROUND_LEVEL, 1.),
                    scale: Vec3::new(SCALE, SCALE, 0.),
                    ..default()
                },
                ..default()
            })
            .insert(Collider::cuboid(TILE_SIZE / 2., TILE_SIZE / 2.));

        for y in -300..=GROUND_LEVEL as i32 {
            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(x as f32 * TILE_SIZE * SCALE, y as f32, 1.),
                    scale: Vec3::new(SCALE, SCALE, 0.),
                    ..default()
                },
                ..default()
            });
        }
    }
}
