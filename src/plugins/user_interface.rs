use crate::systems::user_interface::{
    interactions::{
        deselect_all::deselect_all, select_animal_button::select_animal_button,
        select_animal_menu_button::select_animal_menu_button,
        select_path_button::select_path_button, select_path_menu_button::select_path_menu_button,
        select_rock_button::select_rock_button, select_rock_menu_button::select_rock_menu_button,
        select_terrain_button::select_terrain_button,
        select_terrain_menu_button::select_terrain_menu_button,
        select_tree_button::select_tree_button, select_tree_menu_button::select_tree_menu_button,
        toggle_pause::toggle_pause,
    },
    layouts::{
        despawn_sub_menus::despawn_sub_menus, spawn_animal_menu::spawn_animal_menu,
        spawn_fence_menu::spawn_fence_menu, spawn_path_menu::spawn_path_menu,
        spawn_rock_menu::spawn_rock_menu, spawn_selection_main_menu::spawn_selection_main_menu,
        spawn_terrain_menu::spawn_terrain_menu, spawn_tree_menu::spawn_tree_menu,
    },
};
use bevy::{
    app::Update,
    prelude::{App, Plugin, Startup},
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_selection_main_menu)
            .add_systems(
                Update,
                (
                    spawn_animal_menu,
                    spawn_fence_menu,
                    spawn_terrain_menu,
                    spawn_tree_menu,
                    spawn_rock_menu,
                    spawn_path_menu,
                    deselect_all,
                    despawn_sub_menus,
                    select_animal_menu_button,
                    select_animal_button,
                    // select_fence_menu_button,
                    // select_fence_button,
                    select_terrain_menu_button,
                    select_terrain_button,
                    select_tree_menu_button,
                    select_tree_button,
                    select_rock_menu_button,
                    select_rock_button,
                    // select_shelter_menu_button
                    // select_shelter_button
                    select_path_menu_button,
                    select_path_button,
                    toggle_pause,
                ),
            );
    }
}
