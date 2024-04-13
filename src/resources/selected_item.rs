use bevy::ecs::system::Resource;

use crate::assets::images::{animal::ZooAnimal, world::terrain::WorldTerrain};

#[derive(PartialEq)]
pub enum MainMenuSelection {
    None,
    Animals,
    Fences,
    Terrain,
    Trees,
    Rocks,
    Shelters,
}

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
