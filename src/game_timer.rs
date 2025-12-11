use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct GameTimer {
    pub time: f32,
}

#[derive(Component)]
pub struct GameTimerText;

pub fn setup_text(mut commands: Commands) {
    commands
        .spawn((Text::new("Time: "),))
        .with_child((TextSpan::default(), GameTimerText));
}

pub fn update_timer(time: Res<Time>, mut game_timer: ResMut<GameTimer>) {
    game_timer.time += time.delta_secs();
}

pub fn update_text(
    mut text_query: Query<&mut TextSpan, With<GameTimerText>>,
    game_timer: Res<GameTimer>,
) {
    let mut span = text_query.single_mut().unwrap();
    **span = format!("{:.2}", game_timer.time);
}
