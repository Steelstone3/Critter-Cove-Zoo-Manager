use crate::systems::{
    animate_sprites::animate_sprites,
    user_interface::{
        menu_selection::menu_selection, update_user_interface::update_user_interface,
    },
};
use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, menu_selection)
            .add_systems(Update, update_user_interface)
            .add_systems(Update, animate_sprites);
    }
}
