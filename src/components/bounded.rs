use bevy::prelude::*;

#[derive(Component)]
pub struct Bounded {
    pub min: Option<Vec3>,
    pub max: Option<Vec3>,
}

impl Default for Bounded {
    fn default() -> Self {
        Self {
            min: None,
            max: None,
        }
    }
}
