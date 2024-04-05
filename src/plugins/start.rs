use crate::systems::{camera::add_camera, play_music::play_music, spawn_animal::spawn_animal, spawn_terrain::spawn_terrain};
use bevy::prelude::{App, Plugin, Startup};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(Startup, play_music)
            .add_systems(Startup, spawn_animal)
            .add_systems(Startup, spawn_terrain);
    }
}
