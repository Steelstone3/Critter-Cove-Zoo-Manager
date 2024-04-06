use crate::systems::{
    camera::spawn_camera::spawn_camera, spawn_animal::spawn_animal, spawn_terrain::spawn_terrain,
    user_interface::spawn_user_interface::spawn_user_interface,
};
use bevy::prelude::{App, Plugin, Startup};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_user_interface)
            .add_systems(Startup, spawn_animal)
            .add_systems(Startup, spawn_terrain);
    }
}
