use super::{
    animal_menu::animal_menu, fence_menu::fence_menu, path_menu::path_menu, rock_menu::rock_menu,
    terrain_menu::terrain_menu, tree_menu::tree_menu,
};
use crate::{
    assets::images::user_interface::main_menu::SpawnMenuIcon,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::spawn_menu::SpawnMenu,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn spawn_menu(mut contexts: EguiContexts, mut selected_menu_item: ResMut<SelectedMenuItem>) {
    match selected_menu_item.menu_selection {
        SpawnMenu::None => {
            egui::Window::new("Zoo Manager").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Zoo Animal")).clicked() {
                    selected_menu_item.menu_selection =
                        SpawnMenu::convert_from(SpawnMenuIcon::Animals);
                }
                if ui.add(egui::Button::new("Fences")).clicked() {
                    selected_menu_item.menu_selection =
                        SpawnMenu::convert_from(SpawnMenuIcon::Fences);
                }
                if ui.add(egui::Button::new("Terrain")).clicked() {
                    selected_menu_item.menu_selection =
                        SpawnMenu::convert_from(SpawnMenuIcon::Terrains);
                }
                if ui.add(egui::Button::new("Trees")).clicked() {
                    selected_menu_item.menu_selection =
                        SpawnMenu::convert_from(SpawnMenuIcon::Trees);
                }
                if ui.add(egui::Button::new("Rocks")).clicked() {
                    selected_menu_item.menu_selection =
                        SpawnMenu::convert_from(SpawnMenuIcon::Rocks);
                }
                // Shelters
                if ui.add(egui::Button::new("Paths")).clicked() {
                    selected_menu_item.menu_selection =
                        SpawnMenu::convert_from(SpawnMenuIcon::Paths);
                }
            });
        }

        SpawnMenu::Animals => {
            animal_menu(&mut contexts, &mut selected_menu_item);
        }

        SpawnMenu::Fences => {
            fence_menu(&mut contexts, &mut selected_menu_item);
        }

        SpawnMenu::Terrains => terrain_menu(&mut contexts, &mut selected_menu_item),

        SpawnMenu::Trees => {
            tree_menu(&mut contexts, &mut selected_menu_item);
        }

        SpawnMenu::Rocks => {
            rock_menu(&mut contexts, &mut selected_menu_item);
        }

        SpawnMenu::Paths => {
            path_menu(contexts, selected_menu_item);
        }
    }
}
