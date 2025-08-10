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
            let items = [
                ("Animals", SpawnMenuIcon::Animals),
                ("Fences", SpawnMenuIcon::Fences),
                ("Terrains", SpawnMenuIcon::Terrains),
                ("Trees", SpawnMenuIcon::Trees),
                ("Rocks", SpawnMenuIcon::Rocks),
                ("Paths", SpawnMenuIcon::Paths),
            ];

            if let Ok(ctx) = contexts.ctx_mut() {
                egui::Window::new("Zoo Manager").show(ctx, |ui| {
                    for (label, icon) in items {
                        if ui.add(egui::Button::new(label)).clicked() {
                            selected_menu_item.reset();
                            selected_menu_item.menu_selection = SpawnMenu::convert_from(icon);
                        }
                    }
                });
            } else {
                eprintln!("Terrain Menu failed to render");
            }
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
            path_menu(&mut contexts, &mut selected_menu_item);
        }
    }
}
