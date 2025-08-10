use crate::{
    assets::images::{user_interface::tree_icons::TreeIcon, world::tree_sprites::TreeSprite},
    resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn tree_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
    let items = [
        ("Bush 1", TreeIcon::Bush1),
        ("Bush 2", TreeIcon::Bush2),
        ("Tall Grass 1", TreeIcon::TallGrass1),
        ("Tall Grass 2", TreeIcon::TallGrass2),
        ("Tall Grass 3", TreeIcon::TallGrass3),
        ("Tree 1", TreeIcon::Tree1),
        ("Tree 2", TreeIcon::Tree2),
        ("Tree 3", TreeIcon::Tree3),
        ("Tree 4", TreeIcon::Tree4),
    ];

    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Trees").show(ctx, |ui| {
            for (label, icon) in items {
                if ui.add(egui::Button::new(label)).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.tree_selection = TreeSprite::convert_from(icon);
                }
            }
        });
    } else {
        eprintln!("Tree Menu failed to render");
    }
}
