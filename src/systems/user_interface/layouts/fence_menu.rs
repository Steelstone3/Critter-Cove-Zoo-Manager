use crate::{
    assets::images::world::fence_sprites::FenceSprite, resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn fence_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
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
