use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    assets::images::world::tree_sprites::TreeSprite, resources::selected_item::SelectedMenuItem,
};

pub fn tree_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
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
