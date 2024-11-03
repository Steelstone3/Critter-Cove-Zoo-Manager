use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn spawn_menu(mut contexts: EguiContexts, mut selected_menu_item: ResMut<SelectedMenuItem>) {
    match selected_menu_item.menu_selection {
        // TODO animals, fences, terrain, trees, rocks, (shelters later on), paths
        MainMenuSelection::None => {
            egui::Window::new("Spawn Menu").show(contexts.ctx_mut(), |ui| {
                // ui.label("");

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
        MainMenuSelection::Animals => {}
        MainMenuSelection::Fences => {}
        MainMenuSelection::Terrains => {}
        MainMenuSelection::Trees => {}
        MainMenuSelection::Rocks => {}
        MainMenuSelection::Paths => {}
    }
}
