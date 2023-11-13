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

fn spawn_map(mut commands: Commands) {
    // spawn ground
    commands
        .spawn(SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(0., -50., 0.)),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100., 30.));
}
