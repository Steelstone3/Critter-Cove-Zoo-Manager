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
    resources::selected_item::{self, SelectedMenuItem},
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
            });
        }
        MainMenuSelection::Fences => {
            egui::Window::new("Fences").show(contexts.ctx_mut(), |ui| {
                if ui.add(egui::Button::new("Fence 1")).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.fence_selection = FenceSprite::Fence1;
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
