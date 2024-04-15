use bevy::ecs::system::Resource;

use crate::{
    assets::images::{animal::ZooAnimal, world::terrains::WorldTerrain},
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

#[derive(Resource)]
pub struct SelectedMenuItem {
    pub menu_selection: MainMenuSelection,
    pub animal_selection: ZooAnimal,
    pub terrain_selection: WorldTerrain,
}

impl Default for SelectedMenuItem {
    fn default() -> Self {
        Self {
            menu_selection: MainMenuSelection::None,
            animal_selection: ZooAnimal::None,
            terrain_selection: WorldTerrain::None,
        }
    }
}
