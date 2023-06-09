use crate::player::Player;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_system(animate_player)
            .add_system(change_animation);
    }
}

#[derive(Component, Clone)]
pub struct SpriteAnimation {
    pub starting_index: usize,
    pub len: usize,
    pub frame_time: f32,
}

#[derive(Component, Debug)]
pub struct FrameTime(pub f32);

#[derive(Component, Eq, PartialEq, Hash)]
pub enum Animation {
    Idle,
    Run,
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

// init PlayerAnimations resource with from_world with animation data
impl FromWorld for PlayerAnimations {
    fn from_world(world: &mut World) -> Self {
        let mut map = PlayerAnimations {
            map: HashMap::new(),
        };

        map.add(
            Animation::Idle,
            SpriteAnimation {
                starting_index: 4,
                len: 4,
                frame_time: 0.5,
            },
        );
        map.add(
            Animation::Run,
            SpriteAnimation {
                starting_index: 8,
                len: 8,
                frame_time: 0.3,
            },
        );

        map
    }
}

fn animate_player(
    mut animation: Query<(
        &mut TextureAtlasSprite,
        &mut SpriteAnimation,
        &mut FrameTime,
    )>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in animation.iter_mut() {
        if sprite.index < animation.starting_index {
            sprite.index = animation.starting_index;
        }

        // get time elapsed (f32) since last frame
        frame_time.0 += time.delta_seconds();

        if frame_time.0 > animation.frame_time {
            let frames_elapsed = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames_elapsed;

            // if sprite index becomes greater than length of total animation frames, reset sprite index with modulus
            if sprite.index - animation.starting_index >= animation.len {
                sprite.index %= animation.len;
                sprite.index += animation.starting_index;
            }

            // subtract total frames from frame_time as to not accumulate
            frame_time.0 -= animation.frame_time * frames_elapsed as f32;
        }
    }
}

fn change_animation(
    mut player: Query<(&mut SpriteAnimation, &mut FrameTime), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    animations: Res<PlayerAnimations>,
) {
    let (mut sprite_animation, mut frame_time) = player.single_mut();

    let set_animation = if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        Animation::Run
    } else {
        Animation::Idle
    };

    // get SpriteAnimation data from Animation enum
    let Some(new_animation) = animations.get(set_animation) else {return ();};

    *sprite_animation = new_animation;
}
