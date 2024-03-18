use crate::item::Item;
use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_systems(Update, animate_player)
            .add_systems(Update, change_player_animation)
            .add_systems(Update, animate_item_idle)
            .add_systems(Update, animate_item_in_inv);
    }
}

// Eq, PartialEq, and Hash necessary for animation to be inserted into HashMap world resource.
#[derive(Component, Eq, PartialEq, Hash, Clone, Copy)]
pub enum PlayerAnimationType {
    Idle(Direction),
    Walk(Direction),
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    North,
    East,
    West,
    South,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Component, Clone, Debug)]
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
            PlayerAnimationType::Idle(Direction::South),
            PlayerAnimation {
                len: 6,
                frame_time: 0.13,
                path: "player/idle/idle_south".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Idle(Direction::SouthWest),
            PlayerAnimation {
                len: 4,
                frame_time: 0.1,
                path: "player/idle/idle_southwest".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Idle(Direction::West),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/idle/idle_west".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Idle(Direction::NorthWest),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/idle/idle_northwest".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Idle(Direction::North),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/idle/idle_north".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Idle(Direction::NorthEast),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/idle/idle_northeast".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Idle(Direction::East),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/idle/idle_east".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Idle(Direction::SouthEast),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/idle/idle_southeast".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Walk(Direction::South),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/walk/walk_south".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Walk(Direction::SouthWest),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/walk/walk_southwest".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Walk(Direction::West),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/walk/walk_west".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Walk(Direction::NorthWest),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/walk/walk_northwest".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Walk(Direction::North),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/walk/walk_north".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Walk(Direction::NorthEast),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/walk/walk_northeast".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Walk(Direction::East),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/walk/walk_east".to_string(),
            },
        );

        map.add(
            PlayerAnimationType::Walk(Direction::SouthEast),
            PlayerAnimation {
                len: 6,
                frame_time: 0.1,
                path: "player/walk/walk_southeast".to_string(),
            },
        );

        map
    }
}

// Animation logic for animating player.
fn animate_player(
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite), With<Player>>,
    time: Res<Time>,
    animation_res: Res<PlayerAnimations>,
) {
    for (mut player, mut sprite) in player_query.iter_mut() {
        let Some(animation) = animation_res.get(player.animation) else {
            return;
        };

        player.frame_time += time.delta_seconds();
        if player.frame_time > animation.frame_time {
            let frames_elapsed = player.frame_time / animation.frame_time;

            // Animate!
            sprite.index += frames_elapsed as usize;

            // If sprite index becomes greater than length of total animation frames, reset sprite index. (Restart animation)
            if sprite.index >= animation.len {
                sprite.index %= animation.len;
            }

            // Subtract total frames from frame_time as to not accumulate in frame_time.
            player.frame_time -= animation.frame_time * frames_elapsed as f32;
        }
    }
}

// Change current player animation and spritesheet according to specified logic.
fn change_player_animation(
    mut player_q: Query<&mut Player>,
    item_q: Query<&mut Item>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut texture_atlas_query: Query<&mut Handle<TextureAtlas>, With<Player>>,
    animation_res: Res<PlayerAnimations>,
) {
    let mut player = player_q.single_mut();
    let mut atlas = texture_atlas_query.single_mut();

    let animation_id = if keyboard_input.any_pressed([KeyCode::W, KeyCode::D, KeyCode::S, KeyCode::A, KeyCode::Up, KeyCode::Right, KeyCode::Left, KeyCode::Down]) {
        PlayerAnimationType::Walk(player.direction.clone())
    } else {
        PlayerAnimationType::Idle(player.direction.clone())
    };

    // Get relevant animation and set path accordingly.
    let Some(new_animation) = animation_res.get(animation_id) else {
        return;
    };
    let path = format!("{}.png", &new_animation.path);

    // Load player spritesheet according to relevant path, and splice into single frames. (Why is this so tedious in Bevy?)
    let texture_handle = asset_server.load(path);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(500., 500.), 6, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Set player's spritesheet to relevant data.
    if *atlas != texture_atlas_handle {
        *atlas = texture_atlas_handle
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
    let mut index_num: f32 = 0.;

    for (mut item_pos, item) in item_q.iter_mut() {
        if item.in_inv {
            // Offset to render item in player's hands.
            item_pos.translation.y = player_pos.translation.y - Y_OFFSET;

            // if player.is_facing_right() {
            //     item_pos.translation.x = player_pos.translation.x + X_OFFSET;
            // } else {
            //     item_pos.translation.x = player_pos.translation.x - X_OFFSET;
            // }

            // if player.animation.path == "player/idle" {}
        }
    }
}
