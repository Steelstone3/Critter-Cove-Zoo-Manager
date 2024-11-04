use crate::{
    assets::images::world::path_sprites::PathSprite, resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn path_menu(mut contexts: EguiContexts, mut selected_menu_item: ResMut<SelectedMenuItem>) {
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
