use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component, Default)]
pub struct MoveDirection(pub Vec2);

impl MoveDirection {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.0.x, self.0.y, 0.0)
    }
}

pub fn move_players(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MoveDirection, &Speed), With<Player>>,
) {
    for (mut transform, direction, speed) in query.iter_mut() {
        transform.translation +=
            direction.to_vec3().normalize_or_zero() * speed.0 * time.delta_secs();
    }
}
