use crate::{
    assets::images::world::terrain_sprites::TerrainSprite,
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
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass1;
        }
        if ui.add(egui::Button::new("Dark Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass2;
        }
        if ui.add(egui::Button::new("Dark Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass3;
        }
        if ui.add(egui::Button::new("Dark Grass 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass4;
        }
        if ui.add(egui::Button::new("Dark Grass 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass5;
        }
        if ui.add(egui::Button::new("Dark Grass 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass6;
        }
        if ui.add(egui::Button::new("Dark Grass 7")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass7;
        }
        if ui.add(egui::Button::new("Dark Grass 8")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass8;
        }
        if ui.add(egui::Button::new("Dark Grass 9")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::DarkGrass9;
        }
        if ui.add(egui::Button::new("Grass 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Grass1;
        }
        if ui.add(egui::Button::new("Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Grass2;
        }
        if ui.add(egui::Button::new("Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Grass3;
        }
        if ui.add(egui::Button::new("Grass 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Grass4;
        }
        if ui.add(egui::Button::new("Grass 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Grass5;
        }
        if ui.add(egui::Button::new("Grass 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Grass6;
        }
        if ui.add(egui::Button::new("Light Grass 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::LightGrass1;
        }
        if ui.add(egui::Button::new("Light Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::LightGrass2;
        }
        if ui.add(egui::Button::new("Light Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::LightGrass3;
        }
        if ui.add(egui::Button::new("Light Grass 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::LightGrass4;
        }
        if ui.add(egui::Button::new("Light Grass 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::LightGrass5;
        }
        if ui.add(egui::Button::new("Light Grass 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::LightGrass6;
        }
        if ui.add(egui::Button::new("Savanah 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Savanah1;
        }
        if ui.add(egui::Button::new("Savanah 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Savanah2;
        }
        if ui.add(egui::Button::new("Savanah 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Savanah3;
        }
        if ui.add(egui::Button::new("Savanah 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Savanah4;
        }
        if ui.add(egui::Button::new("Very Light Grass 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::VeryLightGrass1;
        }
        if ui.add(egui::Button::new("Very Light Grass 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::VeryLightGrass2;
        }
        if ui.add(egui::Button::new("Very Light Grass 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::VeryLightGrass3;
        }
        if ui.add(egui::Button::new("Very Light Grass 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::VeryLightGrass4;
        }
        if ui.add(egui::Button::new("Very Light Grass 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::VeryLightGrass5;
        }
        if ui.add(egui::Button::new("Water 1")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water1;
        }
        if ui.add(egui::Button::new("Water 2")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water2;
        }
        if ui.add(egui::Button::new("Water 3")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water3;
        }
        if ui.add(egui::Button::new("Water 4")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water4;
        }
        if ui.add(egui::Button::new("Water 5")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water5;
        }
        if ui.add(egui::Button::new("Water 6")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water6;
        }
        if ui.add(egui::Button::new("Water 7")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water7;
        }
        if ui.add(egui::Button::new("Water 8")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water8;
        }
        if ui.add(egui::Button::new("Water 9")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water9;
        }
        if ui.add(egui::Button::new("Water 10")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water10;
        }
        if ui.add(egui::Button::new("Water 11")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water11;
        }
        if ui.add(egui::Button::new("Water 12")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water12;
        }
        if ui.add(egui::Button::new("Water 13")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water13;
        }
        if ui.add(egui::Button::new("Water 14")).clicked() {
            selected_menu_item.reset();
            selected_menu_item.terrain_selection = TerrainSprite::Water14;
        }
    });
}
