use crate::item::Item;
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_systems(Update, animate_item)
            .add_systems(Update, animate_player)
            .add_systems(Update, change_player_animation)
            .add_systems(Update, flip_sprite);
    }
}

// Eq, PartialEq, and Hash necessary for enum to be inserted into HashMap.
#[derive(Component, Eq, PartialEq, Hash)]
pub enum PlayerAnimationType {
    Idle,
    Run,
    Jump,
    Fall,
}

#[derive(Component, Clone)]
pub struct PlayerAnimation {
    pub len: usize,
    pub frame_time: f32,
    pub path: String,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub map: HashMap<PlayerAnimationType, PlayerAnimation>,
}

impl PlayerAnimations {
    pub fn add(&mut self, id: PlayerAnimationType, animation: PlayerAnimation) {
        self.map.insert(id, animation);
    }

    pub fn get(&self, id: PlayerAnimationType) -> Option<PlayerAnimation> {
        self.map.get(&id).cloned()
    }
}

// Initialize a Bevy resource for player's animations, and add each animation to resource.
impl FromWorld for PlayerAnimations {
    fn from_world(_world: &mut World) -> Self {
        let mut map = PlayerAnimations {
            map: HashMap::new(),
        };

        map.add(
            PlayerAnimationType::Idle,
            PlayerAnimation {
                len: 5,
                frame_time: 0.2,
                path: "player/idle".to_string(),
            },
        );
        map.add(
            PlayerAnimationType::Run,
            PlayerAnimation {
                len: 5,
                frame_time: 0.12,
                path: "player/run".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Jump,
            PlayerAnimation {
                len: 1,
                frame_time: 0.1,
                path: "player/jump".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Fall,
            PlayerAnimation {
                len: 1,
                frame_time: 0.1,
                path: "player/fall".to_string(),
            },
        );

        map
    }
}

// Animation logic for animating item in player's hands. (Swaying animation sort of.)
fn animate_item(
    player_q: Query<&Transform, With<Player>>,
    mut item_q: Query<(&mut Transform, &Item), Without<Player>>,
) {
    let player_pos = player_q.single();
    let (mut item_pos, _) = item_q.single_mut();

    item_pos.translation.x = player_pos.translation.x + 20.;
    item_pos.translation.y = player_pos.translation.y - 10.;
}

// Animation logic for animating player.
fn animate_player(
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite), With<Player>>,
    time: Res<Time>,
) {
    for (mut player, mut sprite) in player_query.iter_mut() {
        player.frame_time += time.delta_seconds();

        if player.frame_time > player.animation.frame_time {
            let frames_elapsed = player.frame_time / player.animation.frame_time;

            // Animate!
            sprite.index += frames_elapsed as usize;

            // If sprite index becomes greater than length of total animation frames, reset sprite index. (To restart animation.)
            if sprite.index >= player.animation.len {
                sprite.index %= player.animation.len;
            }

            // Subtract total frames from frame_time as to not accumulate in frame_time.
            player.frame_time -= player.animation.frame_time * frames_elapsed as f32;
        }
    }
}

// Change current player animation and spritesheet according to specified logic.
fn change_player_animation(
    mut player: Query<&mut Player>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animation_res: Res<PlayerAnimations>,
    mut texture_atlas_query: Query<&mut Handle<TextureAtlas>, With<Player>>,
    velocity: Query<&Velocity, With<Player>>,
) {
    // Cannot simply change jumping and falling animations when velocity is 0, since Bevy Rapier sometimes sets velocity to -0 for some reason.
    const VEL_LIMIT: f32 = 0.01;

    let mut player = player.single_mut();
    let mut atlas = texture_atlas_query.single_mut();
    let vel = velocity.single();

    let curr_animation_id =
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left])
            && !keyboard_input.any_pressed([
                KeyCode::W,
                KeyCode::Up,
                KeyCode::Space,
                KeyCode::S,
                KeyCode::Down,
            ])
            && vel.linvel.y < VEL_LIMIT
            && vel.linvel.y > -VEL_LIMIT
        {
            PlayerAnimationType::Run
        } else if vel.linvel.y > VEL_LIMIT {
            PlayerAnimationType::Jump
        } else if vel.linvel.y < -VEL_LIMIT {
            PlayerAnimationType::Fall
        } else {
            PlayerAnimationType::Idle
        };

    // Get animation object from global animation resource created in FromWorld.
    let Some(new_animation) = animation_res.get(curr_animation_id) else {
        return ();
    };

    // Set path to item spritesheet if player is currently holding item.
    let path = if player.item.is_some() {
        let mut new_path = new_animation.path.clone();
        new_path.push_str("_item.png");
        new_path
    } else {
        let mut new_path = new_animation.path.clone();
        new_path.push_str(".png");
        new_path
    };

    // Load player spritesheet according to relevant path, and splice into single frames. (Why is this so tedious in Bevy?)
    let texture_handle = asset_server.load(path);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Set player's animation and spritesheet to relevant data.
    player.animation = new_animation;
    *atlas = texture_atlas_handle;
}

fn flip_sprite(
    mut player_sprite: Query<&mut TextureAtlasSprite, With<Player>>,
    mut item_q: Query<&mut Transform, With<Item>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut sprite = player_sprite.single_mut();
    let mut item_pos = item_q.single_mut();

    // Flip sprite on x-axis when changing directions.
    // Player sprite spawns facing to the right direction, so flipping when moving left necessary.
    if keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = true;
        item_pos.translation.x -= 20.;
    } else if keyboard_input.any_just_pressed([KeyCode::D, KeyCode::Right])
        && !keyboard_input.any_pressed([KeyCode::A, KeyCode::Left])
    {
        sprite.flip_x = false;
    } else if keyboard_input.any_just_released([KeyCode::A, KeyCode::Left])
        && !keyboard_input.any_pressed([KeyCode::A, KeyCode::Left])
        && keyboard_input.any_pressed([KeyCode::D, KeyCode::Right])
    {
        sprite.flip_x = false;
    }
}
