use crate::{
    assets::images::{
        animal::ZooAnimal,
        world::{
            fences::WorldFence, paths::WorldPath, rocks::WorldRock, terrains::WorldTerrain,
            trees::WorldTree,
        },
    },
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};
use bevy::ecs::system::{ResMut, Resource};

#[derive(Resource)]
pub struct SelectedMenuItem {
    pub menu_selection: MainMenuSelection,
    pub animal_selection: ZooAnimal,
    pub fence_selection: WorldFence,
    pub terrain_selection: WorldTerrain,
    pub tree_selection: WorldTree,
    pub rock_selection: WorldRock,
    // pub shelter_selection: Shelter,
    pub path_selection: WorldPath,
}

impl Default for SelectedMenuItem {
    fn default() -> Self {
        Self {
            menu_selection: MainMenuSelection::None,
            animal_selection: ZooAnimal::None,
            fence_selection: WorldFence::None,
            terrain_selection: WorldTerrain::None,
            tree_selection: WorldTree::None,
            rock_selection: WorldRock::None,
            path_selection: WorldPath::None,
        }
    }
}

impl SelectedMenuItem {
    pub fn reset(selected_item: &mut ResMut<'_, SelectedMenuItem>) {
        selected_item.animal_selection = ZooAnimal::None;
        selected_item.fence_selection = WorldFence::None;
        selected_item.terrain_selection = WorldTerrain::None;
        selected_item.tree_selection = WorldTree::None;
        selected_item.rock_selection = WorldRock::None;
        selected_item.path_selection = WorldPath::None;
    }
}
