use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_idle);
    }
}

#[derive(Component)]
pub struct SpriteAnimation {
    pub len: usize,
    pub frame_time: f32,
}

#[derive(Component, Debug)]
pub struct FrameTime(pub f32);

fn animate_idle(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        // get time elapsed since last frame
        frame_time.0 += time.delta_seconds();

        if frame_time.0 > animation.frame_time {
            let frames_elapsed = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames_elapsed;

            // if sprite index becomes greater than length of total animation frames, reset sprite index with modulus
            if sprite.index >= animation.len {
                sprite.index %= animation.len;
            }

            // subtract total frames from frame_time as to not accumulate
            frame_time.0 -= animation.frame_time * frames_elapsed as f32;
        }
    }
}
