use crate::{
    assets::images::{user_interface::fence_icons::FenceIcon, world::fence_sprites::FenceSprite},
    resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn fence_menu(contexts: &mut EguiContexts, selected_menu_item: &mut ResMut<SelectedMenuItem>) {
    let items = [
        ("Fence1", FenceIcon::Fence1),
        ("Fence2", FenceIcon::Fence2),
        ("Fence3", FenceIcon::Fence3),
        ("Fence4", FenceIcon::Fence4),
    ];

    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Terrain").show(ctx, |ui| {
            for (label, icon) in items {
                if ui.add(egui::Button::new(label)).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.fence_selection = FenceSprite::convert_from(icon);
                }
            }
        });
    } else {
        eprintln!("Fences Menu failed to render");
    }
}
