use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    assets::images::{user_interface::tree_icons::TreeIcon, world::tree_sprites::TreeSprite},
    resources::selected_item::SelectedMenuItem,
};

pub fn tree_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
    egui::Window::new("Trees").show(contexts.ctx_mut().expect("Tree Menu failed to render"), |ui| {
        if ui.add(egui::Button::new("Bush 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::Bush1);
        }
        if ui.add(egui::Button::new("Bush 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::Bush2);
        }
        if ui.add(egui::Button::new("Tall Grass 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::TallGrass1);
        }
        if ui.add(egui::Button::new("Tall Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::TallGrass2);
        }
        if ui.add(egui::Button::new("Tall Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::TallGrass3);
        }
        if ui.add(egui::Button::new("Tree 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::Tree1);
        }
        if ui.add(egui::Button::new("Tree 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::Tree2);
        }
        if ui.add(egui::Button::new("Tree 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::Tree3);
        }
        if ui.add(egui::Button::new("Tree 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.tree_selection = TreeSprite::convert_from(TreeIcon::Tree4);
        }
    });
}
