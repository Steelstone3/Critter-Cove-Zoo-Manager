use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    assets::images::world::{
        fence_sprites::FenceSprite, path_sprites::PathSprite, rock_sprites::RockSprite,
        terrain_sprites::TerrainSprite, tree_sprites::TreeSprite,
    },
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

use super::{animal_menu::animal_menu, fence_menu::fence_menu, terrain_menu::terrain_menu};

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
            egui::Window::new("Trees").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Bush 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::Bush1;
                }
                if ui.add(egui::Button::new("Bush 2")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::Bush2;
                }
                if ui.add(egui::Button::new("Tall Grass 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::TallGrass1;
                }
                if ui.add(egui::Button::new("Tall Grass 2")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::TallGrass2;
                }
                if ui.add(egui::Button::new("Tall Grass 3")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::TallGrass3;
                }
                if ui.add(egui::Button::new("Tree 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::Tree1;
                }
                if ui.add(egui::Button::new("Tree 2")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::Tree2;
                }
                if ui.add(egui::Button::new("Tree 3")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::Tree3;
                }
                if ui.add(egui::Button::new("Tree 4")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::Tree4;
                }
            });
        }

        MainMenuSelection::Rocks => {
            egui::Window::new("Rocks").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Ice Rock 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::IceRock1;
                }
                if ui.add(egui::Button::new("Ice Rock 2")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::IceRock2;
                }
                if ui.add(egui::Button::new("Ice Rock 3")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::IceRock3;
                }
                if ui.add(egui::Button::new("Ice Rock 4")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::IceRock4;
                }
                if ui.add(egui::Button::new("Ice Rock 5")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::IceRock5;
                }
                if ui.add(egui::Button::new("Ice Rock 6")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::IceRock6;
                }
                if ui.add(egui::Button::new("Rock 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock1;
                }
                if ui.add(egui::Button::new("Rock 2")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock2;
                }
                if ui.add(egui::Button::new("Rock 3")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock3;
                }
                if ui.add(egui::Button::new("Rock 4")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock4;
                }
                if ui.add(egui::Button::new("Rock 5")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock5;
                }
                if ui.add(egui::Button::new("Rock 6")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock6;
                }
                if ui.add(egui::Button::new("Rock 7")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock7;
                }
                if ui.add(egui::Button::new("Rock 8")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock8;
                }
                if ui.add(egui::Button::new("Rock 9")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock9;
                }
                if ui.add(egui::Button::new("Rock 10")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock10;
                }
                if ui.add(egui::Button::new("Rock 11")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock11;
                }
                if ui.add(egui::Button::new("Rock 12")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock12;
                }
                if ui.add(egui::Button::new("Rock 13")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock13;
                }
                if ui.add(egui::Button::new("Rock 14")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock14;
                }
                if ui.add(egui::Button::new("Rock 15")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock15;
                }
                if ui.add(egui::Button::new("Rock 16")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock16;
                }
                if ui.add(egui::Button::new("Rock 17")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::Rock17;
                }
                if ui.add(egui::Button::new("Water Rock 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::WaterRock1;
                }
                if ui.add(egui::Button::new("Water Rock 2")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::WaterRock2;
                }
                if ui.add(egui::Button::new("Water Rock 3")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::WaterRock3;
                }
                if ui.add(egui::Button::new("Water Rock 4")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::WaterRock4;
                }
                if ui.add(egui::Button::new("Water Rock 5")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::WaterRock5;
                }
                if ui.add(egui::Button::new("Water Rock 6")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::WaterRock6;
                }
            });
        }

        MainMenuSelection::Paths => {
            egui::Window::new("Paths").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Path 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path1;
                }
                if ui.add(egui::Button::new("Path 2")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path2;
                }
                if ui.add(egui::Button::new("Path 3")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path3;
                }
                if ui.add(egui::Button::new("Path 4")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path4;
                }
                if ui.add(egui::Button::new("Path 5")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path5;
                }
                if ui.add(egui::Button::new("Path 6")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path6;
                }
                if ui.add(egui::Button::new("Path 7")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path7;
                }
                if ui.add(egui::Button::new("Path 8")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path8;
                }
                if ui.add(egui::Button::new("Path 9")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path9;
                }
                if ui.add(egui::Button::new("Path 10")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path10;
                }
                if ui.add(egui::Button::new("Path 11")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path11;
                }
                if ui.add(egui::Button::new("Path 12")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path12;
                }
                if ui.add(egui::Button::new("Path 13")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path13;
                }
                if ui.add(egui::Button::new("Path 14")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path14;
                }
                if ui.add(egui::Button::new("Path 15")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path15;
                }
                if ui.add(egui::Button::new("Path 16")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path16;
                }
                if ui.add(egui::Button::new("Path 17")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path17;
                }
                if ui.add(egui::Button::new("Path 18")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path18;
                }
                if ui.add(egui::Button::new("Path 19")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path19;
                }
                if ui.add(egui::Button::new("Path 20")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path20;
                }
                if ui.add(egui::Button::new("Path 21")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path21;
                }
                if ui.add(egui::Button::new("Path 22")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path22;
                }
                if ui.add(egui::Button::new("Path 23")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path23;
                }
            });
        }
    }
}
