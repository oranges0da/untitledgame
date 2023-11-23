use crate::player::Player;
use crate::Mouse;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_systems(Update, animate_player)
            .add_systems(Update, change_player_animation)
            .add_systems(Update, flip_sprite)
            .add_systems(Update, animate_mouse);
    }
}

// eq, partialeq, and hash necessary for enum to be inserted into HashMap
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

// init PlayerAnimations resource with from_world and add animation data
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

// fundamental animation logic, will be the same for any animation implemented
fn animate_player(
    mut player_query: Query<(&mut Player, &mut TextureAtlasSprite), With<Player>>, // cannot include TextureAtlasSprite in Player{} due to the way Bevy renders entities
    time: Res<Time>,
) {
    for (mut player, mut sprite) in player_query.iter_mut() {
        // get time elapsed (f32) since last frame
        player.frame_time += time.delta_seconds();

        if player.frame_time > player.animation.frame_time {
            let frames_elapsed = player.frame_time / player.animation.frame_time;

            // animate!
            sprite.index += frames_elapsed as usize;

            // if sprite index becomes greater than length of total animation frames, reset sprite index
            if sprite.index >= player.animation.len {
                sprite.index %= player.animation.len;
            }

            // subtract total frames from frame_time as to not accumulate in frame_time
            player.frame_time -= player.animation.frame_time * frames_elapsed as f32;
        }
    }
}

// change player animation and texture_atlas (spritesheet) according to action
fn change_player_animation(
    mut player: Query<&mut Player>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    animations: Res<PlayerAnimations>,
    mut texture_atlas_query: Query<&mut Handle<TextureAtlas>, With<Player>>,
    velocity: Query<&Velocity, With<Player>>,
) {
    const VEL_LIMIT: f32 = 0.2;

    let mut player = player.single_mut();
    let mut atlas = texture_atlas_query.single_mut();
    let vel = velocity.single();

    let curr_animation_id = if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left])
            // to not play running animation when pressing jump and left or right at same time
            && !keyboard_input.any_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space, KeyCode::S, KeyCode::Down]) && vel.linvel.y > -VEL_LIMIT
    {
        PlayerAnimationType::Run
    } else if vel.linvel.y > VEL_LIMIT {
        PlayerAnimationType::Jump
    } else if vel.linvel.y < -VEL_LIMIT {
        // cannot set to 0 due to rapier setting velocity to -0 sometimes for some reason
        PlayerAnimationType::Fall
    } else {
        PlayerAnimationType::Idle
    };

    // get SpriteAnimation data from Animation enum and set accordingly (this is very jerry-rigged for now.)
    let Some(new_animation) = animations.get(curr_animation_id) else {
        return ();
    };

    let path = if player.item.is_some() {
        let mut new_path = new_animation.path.clone();
        new_path.push_str("_item.png");
        new_path
    } else {
        let mut new_path = new_animation.path.clone();
        new_path.push_str(".png");
        new_path
    };

    // load spritesheet and split into grid of individual sprites and convert to spritesheet handle
    let texture_handle = asset_server.load(path);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // set current animation and correct spritesheet
    player.animation = new_animation;
    *atlas = texture_atlas_handle;
}

fn flip_sprite(
    mut player_sprite: Query<&mut TextureAtlasSprite, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut sprite = player_sprite.single_mut();

    // flip sprite on x axis when going from left to right, or vice-verse
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

// draw mouse to cursor's position
fn animate_mouse(
    mut mouse: Query<(&mut Transform, &Mouse), Without<Player>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let (mut mouse_pos, _) = mouse.single_mut();
    let window = q_window.single();

    if let Some(world_pos) = window.cursor_position() {
        // rough coordinates
        mouse_pos.translation.x = world_pos.x - 310.;
        mouse_pos.translation.y = -world_pos.y + 200.;
    }
}
