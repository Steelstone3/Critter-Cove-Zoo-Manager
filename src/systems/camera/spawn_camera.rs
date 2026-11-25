use bevy::{camera::Camera2d, prelude::Commands};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
