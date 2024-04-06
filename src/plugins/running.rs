use crate::systems::{
    animate_sprites::animate_sprites, camera::camera_movement, play_music::play_music, user_interface::{
        menu_selection::menu_selection, update_user_interface::update_user_interface,
    }
};
use bevy::{
    app::Update, ecs::schedule, prelude::{App, Plugin}
};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_music)
            .add_systems(Update, menu_selection)
            .add_systems(Update, update_user_interface)
            .add_systems(Update, animate_sprites) 
            .add_systems(Update, camera_movement);
    }
}
