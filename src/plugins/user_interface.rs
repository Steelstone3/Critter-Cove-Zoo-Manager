use crate::systems::user_interface::{
    interactions::deselect_all::deselect_all, layouts::spawn_menu::spawn_menu,
};
use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_menu)
            .add_systems(Update, deselect_all); // toggle_pause
    }
}
