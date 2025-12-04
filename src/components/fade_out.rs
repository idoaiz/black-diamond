use bevy::prelude::*;

#[derive(Component)]
pub struct FadeOut {
    pub timer: Timer,
}

impl FadeOut {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}
