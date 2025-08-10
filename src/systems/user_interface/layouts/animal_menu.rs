use crate::{
    assets::images::{animal_sprites::AnimalSprite, user_interface::animal_icons::AnimalIcon},
    resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn animal_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
    let items = [
        ("Boar", AnimalIcon::Boar),
        ("Chicken", AnimalIcon::Chicken),
        ("Cow", AnimalIcon::Cow),
        ("Crab", AnimalIcon::Crab),
        ("Dog", AnimalIcon::Dog),
        ("Fox", AnimalIcon::Fox),
        ("Frog", AnimalIcon::Frog),
        ("Goat", AnimalIcon::Goat),
        ("Goose", AnimalIcon::Goose),
        ("Gorilla", AnimalIcon::Gorilla),
        ("Monkey", AnimalIcon::Monkey),
        ("Moose", AnimalIcon::Moose),
        ("Pig", AnimalIcon::Pig),
        ("Porcupine", AnimalIcon::Porcupine),
        ("Sheep", AnimalIcon::Sheep),
        ("Skunk", AnimalIcon::Skunk),
        ("Toad", AnimalIcon::Toad),
        ("Turtle", AnimalIcon::Turtle),
        ("Wolf", AnimalIcon::Wolf),
    ];

    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Animals").show(ctx, |ui| {
            for (label, icon) in items {
                if ui.add(egui::Button::new(label)).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.animal_selection = AnimalSprite::convert_from(icon);
                }
            }
        });
    } else {
        eprintln!("Animals Menu failed to render");
    }
}
