use super::consts::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode, WindowPlugin};

pub fn plugin() -> WindowPlugin {
    let window = Window {
        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
        title: WINDOW_TITLE.to_string(),
        resizable: false,
        mode: WindowMode::Windowed,
        position: WindowPosition::Centered(MonitorSelection::Primary),
        ..default()
    };

    WindowPlugin {
        primary_window: Some(window),
        ..default()
    }
}
