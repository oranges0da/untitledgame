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
            .add_systems(Update, flip_sprite);
    }
}

#[derive(Component, Clone, Debug)]
pub struct SpriteAnimation {
    pub len: usize,
    pub frame_time: f32,
    pub path: String,
}
#[derive(Component, Debug)]
pub struct FrameTime(pub f32);

#[derive(Component, Eq, PartialEq, Hash, Debug)]
pub enum Animation {
    Idle,
    Run,
    Jump,
    Fall,
}

#[derive(Resource)]
pub struct PlayerAnimations {
    pub map: HashMap<Animation, SpriteAnimation>,
}

impl PlayerAnimations {
    pub fn add(&mut self, id: Animation, animation: SpriteAnimation) {
        self.map.insert(id, animation);
    }

    pub fn get(&self, id: Animation) -> Option<SpriteAnimation> {
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
            Animation::Idle,
            SpriteAnimation {
                len: 5,
                frame_time: 0.2,
                path: "player/idle.png".to_string(),
            },
        );
        map.add(
            Animation::Run,
            SpriteAnimation {
                len: 5,
                frame_time: 0.12,
                path: "player/run.png".to_string(),
            },
        );

        map.add(
            Animation::Jump,
            SpriteAnimation {
                len: 1,
                frame_time: 0.,
                path: "player/jump.png".to_string(),
            },
        );

        map.add(
            Animation::Fall,
            SpriteAnimation {
                len: 1,
                frame_time: 0.,
                path: "player/fall.png".to_string(),
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
            let frames_elapsed = (player.frame_time / player.animation.frame_time) as usize;
            sprite.index += frames_elapsed;

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

    let curr_animation = if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right, KeyCode::A, KeyCode::Left])
            // to not play running animation when pressing jump and left or right at same time
            && !keyboard_input.any_pressed([KeyCode::W, KeyCode::Up, KeyCode::Space, KeyCode::S, KeyCode::Down]) && vel.linvel.y > -VEL_LIMIT
    {
        Animation::Run
    } else if vel.linvel.y > VEL_LIMIT {
        Animation::Jump
    } else if vel.linvel.y < -VEL_LIMIT {
        // cannot set to 0 due to rapier setting velocity to -0 sometimes for some reason
        Animation::Fall
    } else {
        Animation::Idle
    };

    // get SpriteAnimation data from Animation enum and set accordingly (this is very jerry-rigged for now.)
    let Some(new_animation) = animations.get(curr_animation) else {
        return ();
    };

    // load spritesheet and split into grid of individual sprites and convert to spritesheet handle
    let texture_handle = asset_server.load(&new_animation.path);
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
