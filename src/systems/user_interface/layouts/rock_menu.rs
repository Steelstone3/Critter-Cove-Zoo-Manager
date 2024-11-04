use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    assets::images::world::rock_sprites::RockSprite, resources::selected_item::SelectedMenuItem,
};

pub fn rock_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
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
