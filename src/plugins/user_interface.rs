use crate::systems::{
    camera::spawn_camera::spawn_camera, spawn_terrain::spawn_world_terrain,
    user_interface::{interactions::{select_tree_button, select_tree_menu_button}, layouts::spawn_selection_main_menu::spawn_selection_main_menu},
};
use bevy::{app::Update, prelude::{App, Plugin, Startup}};

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
    spawn_rock::spawn_rock,
    user_interface::{
        interactions::{
            deselect_all::deselect_all, select_animal_button::select_animal_button,
            select_animal_menu_button::select_animal_menu_button,
            select_rock_button::select_rock_button,
            select_rock_menu_button::select_rock_menu_button,
        },
        layouts::{
            despawn_sub_menus::despawn_sub_menus, spawn_animal_menu::spawn_animal_menu,
            spawn_rock_menu::spawn_rock_menu,
        },
    },
};


pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_selection_main_menu),
        )
        .add_systems(Update, ( spawn_animal_menu,
            spawn_rock_menu,
            deselect_all,
            despawn_sub_menus,
            select_animal_menu_button,
            select_animal_button,
            select_rock_button,
            select_rock_menu_button,
            select_rock_menu_button,
            select_rock_button,));
    }
}
