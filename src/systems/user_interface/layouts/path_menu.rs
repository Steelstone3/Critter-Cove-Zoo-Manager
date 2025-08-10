use crate::{
    assets::images::{user_interface::path_icons::PathIcon, world::path_sprites::PathSprite},
    resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn path_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
    let items = [
        ("Path1", PathIcon::Path1),
        ("Path2", PathIcon::Path2),
        ("Path3", PathIcon::Path3),
        ("Path4", PathIcon::Path4),
        ("Path5", PathIcon::Path5),
        ("Path6", PathIcon::Path6),
        ("Path7", PathIcon::Path7),
        ("Path8", PathIcon::Path8),
        ("Path9", PathIcon::Path9),
        ("Path10", PathIcon::Path10),
        ("Path11", PathIcon::Path11),
        ("Path12", PathIcon::Path12),
        ("Path13", PathIcon::Path13),
        ("Path14", PathIcon::Path14),
        ("Path15", PathIcon::Path15),
        ("Path16", PathIcon::Path16),
        ("Path17", PathIcon::Path17),
        ("Path18", PathIcon::Path18),
        ("Path19", PathIcon::Path19),
        ("Path20", PathIcon::Path20),
        ("Path21", PathIcon::Path21),
        ("Path22", PathIcon::Path22),
        ("Path23", PathIcon::Path23),
    ];

    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Paths").show(ctx, |ui| {
            for (label, icon) in items {
                if ui.add(egui::Button::new(label)).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.path_selection = PathSprite::convert_from(icon);
                }
            }
        });
    } else {
        eprintln!("Paths Menu failed to render");
    }
}
