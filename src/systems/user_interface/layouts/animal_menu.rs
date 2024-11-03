use crate::{assets::images::animals::AnimalSprite, resources::selected_item::SelectedMenuItem};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn animal_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
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
