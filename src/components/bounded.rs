use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Bounded {
    pub min: Option<Vec3>,
    pub max: Option<Vec3>,
}
