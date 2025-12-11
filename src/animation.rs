use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
    pub frames: Vec<usize>,
    pub current_frame_idx: usize,
    pub timer: Timer,
    pub frame_time: f32,
}

impl Animation {
    pub fn new(frames: Vec<usize>, frame_time: f32) -> Self {
        Self {
            frames,
            current_frame_idx: 0,
            timer: Timer::from_seconds(frame_time, TimerMode::Repeating),
            frame_time,
        }
    }

    pub fn next_frame(&mut self) {
        self.current_frame_idx = (self.current_frame_idx + 1) % self.frames.len();
    }
}

pub fn animate(time: Res<Time>, mut query: Query<(&mut Animation, &mut Sprite), With<Player>>) {
    for (mut animation, mut sprite) in query.iter_mut() {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            animation.next_frame();
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation.frames[animation.current_frame_idx];
            }
        }
    }
}
