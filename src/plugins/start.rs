use crate::systems::{
    camera::spawn_camera::spawn_camera, spawn_terrain::spawn_world_terrain,
    user_interface::layouts::spawn_selection_main_menu::spawn_selection_main_menu,
};
use bevy::prelude::{App, Plugin, Startup};

pub struct StartPlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_camera, spawn_selection_main_menu, spawn_world_terrain),
        );
    }
}
