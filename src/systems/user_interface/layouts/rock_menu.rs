use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    assets::images::{user_interface::rock_icons::RockIcon, world::rock_sprites::RockSprite},
    resources::selected_item::SelectedMenuItem,
};

pub fn rock_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
    egui::Window::new("Rocks").show(contexts.ctx_mut(), |ui| {
        if ui.add(egui::Button::new("Ice Rock 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::IceRock1);
        }
        if ui.add(egui::Button::new("Ice Rock 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::IceRock2);
        }
        if ui.add(egui::Button::new("Ice Rock 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::IceRock3);
        }
        if ui.add(egui::Button::new("Ice Rock 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::IceRock4);
        }
        if ui.add(egui::Button::new("Ice Rock 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::IceRock5);
        }
        if ui.add(egui::Button::new("Ice Rock 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::IceRock6);
        }
        if ui.add(egui::Button::new("Rock 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock1);
        }
        if ui.add(egui::Button::new("Rock 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock2);
        }
        if ui.add(egui::Button::new("Rock 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock3);
        }
        if ui.add(egui::Button::new("Rock 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock4);
        }
        if ui.add(egui::Button::new("Rock 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock5);
        }
        if ui.add(egui::Button::new("Rock 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock6);
        }
        if ui.add(egui::Button::new("Rock 7")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock7);
        }
        if ui.add(egui::Button::new("Rock 8")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock8);
        }
        if ui.add(egui::Button::new("Rock 9")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock9);
        }
        if ui.add(egui::Button::new("Rock 10")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock10);
        }
        if ui.add(egui::Button::new("Rock 11")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock11);
        }
        if ui.add(egui::Button::new("Rock 12")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock12);
        }
        if ui.add(egui::Button::new("Rock 13")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock13);
        }
        if ui.add(egui::Button::new("Rock 14")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock14);
        }
        if ui.add(egui::Button::new("Rock 15")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock15);
        }
        if ui.add(egui::Button::new("Rock 16")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock16);
        }
        if ui.add(egui::Button::new("Rock 17")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::Rock17);
        }
        if ui.add(egui::Button::new("Water Rock 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::WaterRock1);
        }
        if ui.add(egui::Button::new("Water Rock 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::WaterRock2);
        }
        if ui.add(egui::Button::new("Water Rock 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::WaterRock3);
        }
        if ui.add(egui::Button::new("Water Rock 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::WaterRock4);
        }
        if ui.add(egui::Button::new("Water Rock 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::WaterRock5);
        }
        if ui.add(egui::Button::new("Water Rock 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.rock_selection = RockSprite::convert_from(RockIcon::WaterRock6);
        }
    });
}
