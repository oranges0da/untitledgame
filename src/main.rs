use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;

mod animation;
mod debug;
mod item;
mod map;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest()) // necessary to not spawn blurry sprites
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Untitled Game".into(),
                        resolution: (640., 480.).into(),
                        present_mode: PresentMode::AutoVsync,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.))) // set background color to black (outer space!)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_mouse)
        .add_systems(Update, follow_player)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(debug::DebugPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(item::ItemPlugin)
        .run();
}

#[derive(Component)]
struct PlayerCamera;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PlayerCamera));
}

// camera follows player (player is always centered on screen)
fn follow_player(
    player_query: Query<&Transform, With<player::Player>>,
    mut camera: Query<(&PlayerCamera, &mut Transform), Without<player::Player>>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };

    let Ok((_, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    // set camera's coordinates to player's coordinates on each frame
    camera_transform.translation = player.translation;
}

#[derive(Component)]
pub struct Mouse;

// draw mouse sprite (pointer) on mouse pos
fn spawn_mouse(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // load spritesheet and split into grid of individual sprites and convert to spritesheet handle
    let texture_handle = asset_server.load("ui/mouse.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 4, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_scale(Vec3::new(1.5, 1.5, 0.)),
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            ..default()
        })
        .insert(Mouse);
}
