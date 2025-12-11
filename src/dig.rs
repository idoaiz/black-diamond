use crate::components::fade_out::FadeOut;
use crate::player::{Player, PlayerConfig};
use bevy::prelude::*;

#[derive(Message)]
pub struct DigEvent {
    pub actor: Entity,
    pub location: Vec3,
}
#[derive(Component)]
pub struct DigCooldown {
    timer: Timer,
}

impl DigCooldown {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
        }
    }
}

/// Update the dig cooldown for all entities that currently have it.
/// When cooldown ends, remove the component from the entity.
pub fn dig_cooldown(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DigCooldown)>,
) {
    for (entity, mut cooldown) in &mut query {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.is_finished() {
            commands.entity(entity).remove::<DigCooldown>();
        }
    }
}

/// A system to catch dig actions by player and execute them
pub fn dig(
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    player_config: Res<PlayerConfig>,
    mut commands: Commands,
    mut dig_message_writer: MessageWriter<DigEvent>,
    query: Query<(Entity, &mut Transform, Option<&DigCooldown>), With<Player>>,
) {
    let (entity, transform, cooldown) = query.single().unwrap();

    if keyboard.pressed(KeyCode::Space) && cooldown.is_none() {
        let translation = transform.translation;
        commands.spawn((
            Sprite::from_image(asset_server.load("effects/hole.png")),
            FadeOut::new(1.),
            Transform::from_xyz(translation.x, translation.y, translation.z - 0.1),
        ));

        commands
            .entity(entity)
            .insert(DigCooldown::new(player_config.dig_cooldown));

        dig_message_writer.write(DigEvent {
            actor: entity,
            location: translation,
        });
    }
}
