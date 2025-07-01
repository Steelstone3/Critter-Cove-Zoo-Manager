use crate::{
    assets::images::{animal_sprites::AnimalSprite, user_interface::animal_icons::AnimalIcon},
    resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn animal_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
    egui::Window::new("Animals").show(contexts.ctx_mut().expect("Animals Menu failed to render"), |ui| {
        if ui.add(egui::Button::new("Boar")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Boar);
        }
        if ui.add(egui::Button::new("Chicken")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Chicken);
        }
        if ui.add(egui::Button::new("Cow")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Cow);
        }
        if ui.add(egui::Button::new("Crab")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Crab);
        }
        if ui.add(egui::Button::new("Dog")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Dog);
        }
        if ui.add(egui::Button::new("Fox")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Fox);
        }
        if ui.add(egui::Button::new("Frog")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Frog);
        }
        if ui.add(egui::Button::new("Goat")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Goat);
        }
        if ui.add(egui::Button::new("Goose")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Goose);
        }
        if ui.add(egui::Button::new("Gorilla")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Gorilla);
        }
        if ui.add(egui::Button::new("Monkey")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Monkey);
        }
        if ui.add(egui::Button::new("Moose")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Moose);
        }
        if ui.add(egui::Button::new("Pig")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Pig);
        }
        if ui.add(egui::Button::new("Porcupine")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Porcupine);
        }
        if ui.add(egui::Button::new("Sheep")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Sheep);
        }
        if ui.add(egui::Button::new("Skunk")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Skunk);
        }
        if ui.add(egui::Button::new("Toad")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Toad);
        }
        if ui.add(egui::Button::new("Turtle")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Turtle);
        }
        if ui.add(egui::Button::new("Wolf")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.animal_selection = AnimalSprite::convert_from(AnimalIcon::Wolf);
        }
    });
}
