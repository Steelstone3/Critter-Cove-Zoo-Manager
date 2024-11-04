use crate::{
    assets::images::{user_interface::terrain_icons::TerrainIcon, world::terrain_sprites::TerrainSprite},
    resources::selected_item::SelectedMenuItem,
};
use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};

pub fn terrain_menu(
    contexts: &mut EguiContexts,
    selected_menu_item: &mut ResMut<SelectedMenuItem>,
) {
    egui::Window::new("Terrains").show(contexts.ctx_mut(), |ui| {
        if ui.add(egui::Button::new("Dark Grass 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass1);
        }
        if ui.add(egui::Button::new("Dark Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass2);
        }
        if ui.add(egui::Button::new("Dark Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass3);
        }
        if ui.add(egui::Button::new("Dark Grass 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass4);
        }
        if ui.add(egui::Button::new("Dark Grass 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass5);
        }
        if ui.add(egui::Button::new("Dark Grass 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass6);
        }
        if ui.add(egui::Button::new("Dark Grass 7")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass7);
        }
        if ui.add(egui::Button::new("Dark Grass 8")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass8);
        }
        if ui.add(egui::Button::new("Dark Grass 9")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::DarkGrass9);
        }
        if ui.add(egui::Button::new("Grass 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Grass1);
        }
        if ui.add(egui::Button::new("Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Grass2);
        }
        if ui.add(egui::Button::new("Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Grass3);
        }
        if ui.add(egui::Button::new("Grass 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Grass4);
        }
        if ui.add(egui::Button::new("Grass 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Grass5);
        }
        if ui.add(egui::Button::new("Grass 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Grass6);
        }
        if ui.add(egui::Button::new("Light Grass 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::LightGrass1);
        }
        if ui.add(egui::Button::new("Light Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::LightGrass2);
        }
        if ui.add(egui::Button::new("Light Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::LightGrass3);
        }
        if ui.add(egui::Button::new("Light Grass 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::LightGrass4);
        }
        if ui.add(egui::Button::new("Light Grass 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::LightGrass5);
        }
        if ui.add(egui::Button::new("Light Grass 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::LightGrass6);
        }
        if ui.add(egui::Button::new("Savanah 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Savanah1);
        }
        if ui.add(egui::Button::new("Savanah 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Savanah2);
        }
        if ui.add(egui::Button::new("Savanah 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Savanah3);
        }
        if ui.add(egui::Button::new("Savanah 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Savanah4);
        }
        if ui.add(egui::Button::new("Very Light Grass 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::VeryLightGrass1);
        }
        if ui.add(egui::Button::new("Very Light Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::VeryLightGrass2);
        }
        if ui.add(egui::Button::new("Very Light Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::VeryLightGrass3);
        }
        if ui.add(egui::Button::new("Very Light Grass 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::VeryLightGrass4);
        }
        if ui.add(egui::Button::new("Very Light Grass 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::VeryLightGrass5);
        }
        if ui.add(egui::Button::new("Water 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water1);
        }
        if ui.add(egui::Button::new("Water 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water2);
        }
        if ui.add(egui::Button::new("Water 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water3);
        }
        if ui.add(egui::Button::new("Water 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water4);
        }
        if ui.add(egui::Button::new("Water 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water5);
        }
        if ui.add(egui::Button::new("Water 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water6);
        }
        if ui.add(egui::Button::new("Water 7")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water7);
        }
        if ui.add(egui::Button::new("Water 8")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water8);
        }
        if ui.add(egui::Button::new("Water 9")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water9);
        }
        if ui.add(egui::Button::new("Water 10")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water10);
        }
        if ui.add(egui::Button::new("Water 11")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water11);
        }
        if ui.add(egui::Button::new("Water 12")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water12);
        }
        if ui.add(egui::Button::new("Water 13")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water13);
        }
        if ui.add(egui::Button::new("Water 14")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::convert_from(TerrainIcon::Water14);
        }
    });
}
