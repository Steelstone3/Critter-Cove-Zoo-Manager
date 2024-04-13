use crate::systems::{
    animal_movement::animal_movement,
    animate_sprites::animate_sprites,
    camera::{
        camera_movement::camera_movement, camera_position_reset::camera_position_reset,
        camera_zoom_keyboard::camera_zoom_keyboard,
        camera_zoom_mouse_and_touchpad::camera_zoom_mouse_and_touchpad,
    },
    play_music::play_music,
    spawn_animal::spawn_animal,
    user_interface::layout::spawn_selection_menu::spawn_selection_menu,
};
use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_movement)
            .add_systems(Update, camera_position_reset)
            .add_systems(Update, camera_zoom_keyboard)
            .add_systems(Update, camera_zoom_mouse_and_touchpad)
            .add_systems(Update, play_music)
            .add_systems(Update, spawn_selection_menu)
            .add_systems(Update, animate_sprites)
            .add_systems(Update, spawn_animal)
            .add_systems(Update, animal_movement);
    }
}
