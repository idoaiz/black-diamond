use crate::components::fade_out::FadeOut;
use bevy::prelude::*;

pub fn fade_out(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite, &mut FadeOut)>,
) {
    for (entity, mut sprite, mut fade_out) in query.iter_mut() {
        fade_out.timer.tick(time.delta());

        sprite.color.set_alpha(fade_out.timer.fraction_remaining());

        if fade_out.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
