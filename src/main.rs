use bevy::prelude::*;

mod components {
    pub mod ui;
}

use components::ui::spawn_item_grid;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: [800., 600.].into(),
            title: "Modular Inventory".to_string(),
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, spawn_camera)
    .add_systems(Startup, spawn_item_grid)
    .run();
}

fn spawn_camera (mut commands : Commands) {
    commands.spawn(Camera2dBundle{
        ..default()
    });
}
