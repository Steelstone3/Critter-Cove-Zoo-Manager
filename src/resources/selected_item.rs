use crate::{
    assets::images::{
        animal::ZooAnimal,
        world::{rocks::WorldRock, terrains::WorldTerrain, trees::WorldTree},
    },
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct SelectedMenuItem {
    pub menu_selection: MainMenuSelection,
    pub animal_selection: ZooAnimal,
    // pub fence_selection: Fence,
    pub terrain_selection: WorldTerrain,
    pub tree_selection: WorldTree,
    pub rock_selection: WorldRock,
    // pub shelter_selection: Shelter,
    // pub path_selection: Path,
}

impl Default for SelectedMenuItem {
    fn default() -> Self {
        Self {
            menu_selection: MainMenuSelection::None,
            animal_selection: ZooAnimal::None,
            terrain_selection: WorldTerrain::None,
            tree_selection: WorldTree::None,
            rock_selection: WorldRock::None,
        }
    }
}
