use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    assets::images::{
        animals::AnimalSprite,
        world::{
            fence_sprites::FenceSprite, path_sprites::PathSprite, rock_sprites::RockSprite,
            terrain_sprites::TerrainSprite, tree_sprites::TreeSprite,
        },
    },
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
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
        // TODO animals, fences, terrain, trees, rocks, (shelters later on), paths
        MainMenuSelection::Animals => {
            egui::Window::new("Animals").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Boar")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Boar;
                }
                if ui.add(egui::Button::new("Chicken")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Chicken;
                }
                if ui.add(egui::Button::new("Cow")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Cow;
                }
                if ui.add(egui::Button::new("Crab")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Crab;
                }
                if ui.add(egui::Button::new("Dog")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Dog;
                }
                if ui.add(egui::Button::new("Fox")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Fox;
                }
                if ui.add(egui::Button::new("Frog")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Frog;
                }
                if ui.add(egui::Button::new("Goat")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Goat;
                }
                if ui.add(egui::Button::new("Goose")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Goose;
                }
                if ui.add(egui::Button::new("Monkey")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Monkey;
                }
                if ui.add(egui::Button::new("Pig")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Pig;
                }
                if ui.add(egui::Button::new("Porcupine")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Porcupine;
                }
                if ui.add(egui::Button::new("Sheep")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Sheep;
                }
                if ui.add(egui::Button::new("Skunk")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Skunk;
                }
                if ui.add(egui::Button::new("Toad")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Toad;
                }
                if ui.add(egui::Button::new("Turtle")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Turtle;
                }
                if ui.add(egui::Button::new("Wolf")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::Wolf;
                }
            });
        }
        MainMenuSelection::Fences => {
            egui::Window::new("Fences").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Fence 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.fence_selection = FenceSprite::Fence1;
                }
                if ui.add(egui::Button::new("Fence 2")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.fence_selection = FenceSprite::Fence2;
                }
                if ui.add(egui::Button::new("Fence 3")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.fence_selection = FenceSprite::Fence3;
                }
                if ui.add(egui::Button::new("Fence 4")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.fence_selection = FenceSprite::Fence4;
                }
            });
        }
        MainMenuSelection::Terrains => {
            egui::Window::new("Terrains").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Dark Grass 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.terrain_selection = TerrainSprite::DarkGrass1;
                }
            });
        }
        MainMenuSelection::Trees => {
            egui::Window::new("Trees").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Bush 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::Bush1;
                }
            });
        }
        MainMenuSelection::Rocks => {
            egui::Window::new("Rocks").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Ice Rock 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::IceRock1;
                }
            });
        }
        MainMenuSelection::Paths => {
            egui::Window::new("Paths").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Path 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::Path1;
                }
            });
        }
    }
}
