use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

use crate::{
    assets::images::{user_interface::rock_icons::RockIcon, world::rock_sprites::RockSprite},
    resources::selected_item::SelectedMenuItem,
};

pub fn rock_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
    let items = [
        ("IceRock1", RockIcon::IceRock1),
        ("IceRock2", RockIcon::IceRock2),
        ("IceRock3", RockIcon::IceRock3),
        ("IceRock4", RockIcon::IceRock4),
        ("IceRock5", RockIcon::IceRock5),
        ("IceRock6", RockIcon::IceRock6),
        ("Rock1", RockIcon::Rock1),
        ("Rock2", RockIcon::Rock2),
        ("Rock3", RockIcon::Rock3),
        ("Rock4", RockIcon::Rock4),
        ("Rock5", RockIcon::Rock5),
        ("Rock6", RockIcon::Rock6),
        ("Rock7", RockIcon::Rock7),
        ("Rock8", RockIcon::Rock8),
        ("Rock9", RockIcon::Rock9),
        ("Rock10", RockIcon::Rock10),
        ("Rock11", RockIcon::Rock11),
        ("Rock12", RockIcon::Rock12),
        ("Rock13", RockIcon::Rock13),
        ("Rock14", RockIcon::Rock14),
        ("Rock15", RockIcon::Rock15),
        ("Rock16", RockIcon::Rock16),
        ("Rock17", RockIcon::Rock17),
        ("WaterRock1", RockIcon::WaterRock1),
        ("WaterRock2", RockIcon::WaterRock2),
        ("WaterRock3", RockIcon::WaterRock3),
        ("WaterRock4", RockIcon::WaterRock4),
        ("WaterRock5", RockIcon::WaterRock5),
        ("WaterRock6", RockIcon::WaterRock6),
    ];

    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Rocks").show(ctx, |ui| {
            for (label, icon) in items {
                if ui.add(egui::Button::new(label)).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.rock_selection = RockSprite::convert_from(icon);
                }
            }
        });
    } else {
        eprintln!("Rocks Menu failed to render");
    }
}
