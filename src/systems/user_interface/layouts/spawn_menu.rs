use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

use super::{
    animal_menu::animal_menu, fence_menu::fence_menu, path_menu::path_menu, rock_menu::rock_menu,
    terrain_menu::terrain_menu, tree_menu::tree_menu,
};

pub fn spawn_menu(mut contexts: EguiContexts, mut selected_menu_item: ResMut<SelectedMenuItem>) {
    match selected_menu_item.menu_selection {
        MainMenuSelection::None => {
            egui::Window::new("Zoo Manager").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Zoo Animal")).clicked() {
                    selected_menu_item.menu_selection = MainMenuSelection::Animals;
                }
                if ui.add(egui::Button::new("Fences")).clicked() {
                    selected_menu_item.menu_selection = MainMenuSelection::Fences;
                }
                if ui.add(egui::Button::new("Terrain")).clicked() {
                    selected_menu_item.menu_selection = MainMenuSelection::Terrains;
                }
                if ui.add(egui::Button::new("Trees")).clicked() {
                    selected_menu_item.menu_selection = MainMenuSelection::Trees;
                }
                if ui.add(egui::Button::new("Rocks")).clicked() {
                    selected_menu_item.menu_selection = MainMenuSelection::Rocks;
                }
                // Shelters
                if ui.add(egui::Button::new("Paths")).clicked() {
                    selected_menu_item.menu_selection = MainMenuSelection::Paths;
                }
            });
        }

        MainMenuSelection::Animals => {
            animal_menu(&mut contexts, &mut selected_menu_item);
        }

        MainMenuSelection::Fences => {
            fence_menu(&mut contexts, &mut selected_menu_item);
        }

        MainMenuSelection::Terrains => terrain_menu(&mut contexts, &mut selected_menu_item),

        MainMenuSelection::Trees => {
            tree_menu(&mut contexts, &mut selected_menu_item);
        }

        MainMenuSelection::Rocks => {
            rock_menu(&mut contexts, &mut selected_menu_item);
        }

        MainMenuSelection::Paths => {
            path_menu(contexts, selected_menu_item);
        }
    }
}
