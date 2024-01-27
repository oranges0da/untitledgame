use crate::item::Item;
use crate::map::GroundTile;
use crate::player::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_systems(Update, animate_player)
            .add_systems(Update, change_player_animation)
            .add_systems(Update, flip_player)
            .add_systems(Update, animate_item_idle)
            .add_systems(Update, animate_item_in_inv);
    }
}

// Eq, PartialEq, and Hash necessary for animation to be inserted into HashMap world resource.
#[derive(Component, Eq, PartialEq, Hash)]
pub enum PlayerAnimationType {
    Idle,
    Run,
    Jump,
}

#[derive(Component, Clone, Debug)]
pub struct PlayerAnimation {
    pub len: usize,
    pub frame_time: f32,
    pub path: String,
    pub tile_size: Vec2,
    pub rows: usize,
    pub columns: usize,
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
                len: 7,
                frame_time: 0.1,
                path: "player/idle".to_string(),
                tile_size: Vec2::new(32., 32.),
                rows: 1,
                columns: 8,
            },
        );
        map.add(
            PlayerAnimationType::Run,
            PlayerAnimation {
                len: 7,
                frame_time: 0.1,
                path: "player/run".to_string(),
                tile_size: Vec2::new(32., 32.),
                rows: 1,
                columns: 8,
            },
        );

        map.add(
            PlayerAnimationType::Jump,
            PlayerAnimation {
                len: 3,
                frame_time: 0.1,
                path: "player/jump".to_string(),
                tile_size: Vec2::new(32., 32.),
                rows: 1,
                columns: 4,
            },
        );

        map
    }
}

// Animation logic for animating player.
fn animate_player(
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite), With<Player>>,
    time: Res<Time>,
) {
    for (mut player, mut sprite) in player_query.iter_mut() {
        info!("Sprite index: {:?}", sprite.index);
        player.frame_time += time.delta_seconds();

        if player.frame_time > player.animation.frame_time {
            let frames_elapsed = player.frame_time / player.animation.frame_time;

            // Animate!
            sprite.index += frames_elapsed as usize;

            // If sprite index is length of animation, (animation is finished)
            // reset sprite index. (Restart animation.)
            if sprite.index == player.animation.len {
                sprite.index = 0;
            }

            // Subtract total frames from frame_time as to not accumulate in frame_time.
            player.frame_time -= player.animation.frame_time * frames_elapsed as f32;
        }
    }
}

// Change current player animation and spritesheet according to specified logic.
fn change_player_animation(
    mut player_q: Query<&mut Player>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlas_query: Query<&mut Handle<TextureAtlas>, With<Player>>,
    vel_q: Query<&Velocity, With<Player>>,
    animation_res: Res<PlayerAnimations>,
) {
    // Cannot simply change jumping and falling animations when velocity is 0, since Bevy Rapier sometimes sets velocity to -0 for some reason.
    const VEL_LIMIT: f32 = 0.02;

    let mut player = player_q.single_mut();
    let mut atlas = texture_atlas_query.single_mut();
    let vel = vel_q.single();

    let curr_animation_id =
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left])
            && !keyboard_input.any_pressed([
                KeyCode::W,
                KeyCode::Up,
                KeyCode::Space,
                KeyCode::S,
                KeyCode::Down,
            ])
        // Velocity is somewhere between -0.02 and 0.02, which is standing still on the y-axis in the eyes of Rapier.
        // && vel.linvel.y < VEL_LIMIT && vel.linvel.y > -VEL_LIMIT
        {
            PlayerAnimationType::Run
        } else if vel.linvel.y > VEL_LIMIT
            && keyboard_input.any_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space])
        {
            PlayerAnimationType::Jump
        } else {
            PlayerAnimationType::Idle
        };

    // Get relevant animation and set path accordingly.
    let Some(new_animation) = animation_res.get(curr_animation_id) else {
        return;
    };
    let path = format!("{}.png", new_animation.path);

    // Load player spritesheet according to relevant path, and splice into single frames. (Why is this so tedious in Bevy?)
    let texture_handle = asset_server.load(path.clone());
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        new_animation.tile_size,
        new_animation.columns,
        new_animation.rows,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Set player's animation and spritesheet to relevant data.
    player.animation = new_animation;
    *atlas = texture_atlas_handle
}

fn flip_player(
    mut player_sprite: Query<&mut TextureAtlasSprite, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut sprite = player_sprite.single_mut();

    // Flip sprite on x-axis when changing directions.
    // Player sprite spawns facing to the right direction, so flipping when moving left necessary.
    if keyboard_input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = true;
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

// Animate idle item on floor.
fn animate_item_idle(
    mut item_q: Query<&mut Transform, With<Item>>,
    mut frame_time: Local<i32>,
    mut switch: Local<i32>,
) {
    const ANIM_LIMIT: i32 = 20; // Limit for top of animation.
    const STEP: f32 = 0.2; // How much to increase position on each frame.

    if *frame_time < ANIM_LIMIT && *switch == 0 {
        *frame_time += 1;
    } else if *frame_time >= ANIM_LIMIT && *switch == 0 {
        *frame_time = 0;
        *switch = 1;
    } else if *frame_time >= -ANIM_LIMIT && *switch == 1 {
        *frame_time -= 1;
    } else {
        *switch = 0;
        *frame_time = 0;
    }

    for mut pos in &mut item_q.iter_mut() {
        if *switch == 0 {
            pos.translation.y += STEP; // Going up.
        } else if *switch == 1 {
            pos.translation.y -= STEP; // Going down.
        }
    }
}

fn animate_item_in_inv(
    player_q: Query<(&Player, &Transform, &mut TextureAtlasSprite), With<Player>>,
    mut item_q: Query<(&mut Transform, &Item), Without<Player>>,
) {
    const X_OFFSET: f32 = 20.;
    const Y_OFFSET: f32 = 5.;

    let (player, player_pos, sprite) = player_q.single();

    for (mut item_pos, item) in item_q.iter_mut() {
        if item.in_inv {
            // Offset to render item in player's hands.
            item_pos.translation.y = player_pos.translation.y - Y_OFFSET;

            if player.is_facing_right() {
                item_pos.translation.x = player_pos.translation.x + X_OFFSET;
            } else {
                item_pos.translation.x = player_pos.translation.x - X_OFFSET;
            }
        }
    }
}
