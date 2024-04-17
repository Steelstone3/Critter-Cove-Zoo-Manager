use crate::systems::{
    camera::spawn_camera::spawn_camera, spawn_world_terrain::spawn_world_terrain,
};
use bevy::prelude::{App, Plugin, Startup};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_world_terrain));
    }
}
