use bevy::{core_pipeline::core_2d::Camera2d, prelude::Commands};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
