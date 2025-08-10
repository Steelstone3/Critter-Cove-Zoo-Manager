use crate::{
    assets::images::{
        user_interface::terrain_icons::TerrainIcon, world::terrain_sprites::TerrainSprite,
    },
    resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn terrain_menu(
    contexts: &mut EguiContexts,
    selected_menu_item: &mut ResMut<SelectedMenuItem>,
) {
    let items = [
        ("Dark Grass 1", TerrainIcon::DarkGrass1),
        ("DarkGrass2", TerrainIcon::DarkGrass2),
        ("DarkGrass3", TerrainIcon::DarkGrass3),
        ("DarkGrass4", TerrainIcon::DarkGrass4),
        ("DarkGrass5", TerrainIcon::DarkGrass5),
        ("DarkGrass6", TerrainIcon::DarkGrass6),
        ("DarkGrass7", TerrainIcon::DarkGrass7),
        ("DarkGrass8", TerrainIcon::DarkGrass8),
        ("DarkGrass9", TerrainIcon::DarkGrass9),
        ("Grass1", TerrainIcon::Grass1),
        ("Grass2", TerrainIcon::Grass2),
        ("Grass3", TerrainIcon::Grass3),
        ("Grass4", TerrainIcon::Grass4),
        ("Grass5", TerrainIcon::Grass5),
        ("Grass6", TerrainIcon::Grass6),
        ("LightGrass1", TerrainIcon::LightGrass1),
        ("LightGrass2", TerrainIcon::LightGrass2),
        ("LightGrass3", TerrainIcon::LightGrass3),
        ("LightGrass4", TerrainIcon::LightGrass4),
        ("LightGrass5", TerrainIcon::LightGrass5),
        ("LightGrass6", TerrainIcon::LightGrass6),
        ("Savanah1", TerrainIcon::Savanah1),
        ("Savanah2", TerrainIcon::Savanah2),
        ("Savanah3", TerrainIcon::Savanah3),
        ("Savanah4", TerrainIcon::Savanah4),
        ("VeryLightGrass1", TerrainIcon::VeryLightGrass1),
        ("VeryLightGrass2", TerrainIcon::VeryLightGrass2),
        ("VeryLightGrass3", TerrainIcon::VeryLightGrass3),
        ("VeryLightGrass4", TerrainIcon::VeryLightGrass4),
        ("VeryLightGrass5", TerrainIcon::VeryLightGrass5),
        ("Water1", TerrainIcon::Water1),
        ("Water2", TerrainIcon::Water2),
        ("Water3", TerrainIcon::Water3),
        ("Water4", TerrainIcon::Water4),
        ("Water5", TerrainIcon::Water5),
        ("Water6", TerrainIcon::Water6),
        ("Water7", TerrainIcon::Water7),
        ("Water8", TerrainIcon::Water8),
        ("Water9", TerrainIcon::Water9),
        ("Water10", TerrainIcon::Water10),
        ("Water11", TerrainIcon::Water11),
        ("Water12", TerrainIcon::Water12),
        ("Water13", TerrainIcon::Water13),
        ("Water 14", TerrainIcon::Water14),
    ];

    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Terrain").show(ctx, |ui| {
            for (label, icon) in items {
                if ui.add(egui::Button::new(label)).clicked() {
                    selected_menu_item.reset();
                    selected_menu_item.terrain_selection = TerrainSprite::convert_from(icon);
                }
            }
        });
    } else {
        eprintln!("Terrain Menu failed to render");
    }
}
