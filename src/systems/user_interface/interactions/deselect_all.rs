use bevy::{
    ecs::system::ResMut,
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::{
    assets::images::{
        animal::ZooAnimal,
        world::{rocks::WorldRock, terrains::WorldTerrain, trees::WorldTree},
    },
    resources::selected_item::SelectedMenuItem,
};

use super::main_menu_selection::MainMenuSelection;

pub fn deselect_all(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut selected_item: ResMut<SelectedMenuItem>,
) {
    if input.clear_just_pressed(KeyCode::Escape) {
        selected_item.menu_selection = MainMenuSelection::None;
        selected_item.animal_selection = ZooAnimal::None;
        selected_item.terrain_selection = WorldTerrain::None;
        selected_item.rock_selection = WorldRock::None;
        selected_item.tree_selection = WorldTree::None;
    }
}
