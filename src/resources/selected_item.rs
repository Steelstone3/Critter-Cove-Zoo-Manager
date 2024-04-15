use crate::{
    assets::images::{
        animal::ZooAnimal,
        world::{rocks::WorldRock, terrains::WorldTerrain},
    },
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct SelectedMenuItem {
    pub menu_selection: MainMenuSelection,
    pub animal_selection: ZooAnimal,
    pub terrain_selection: WorldTerrain,
    pub rock_selection: WorldRock,
}

impl Default for SelectedMenuItem {
    fn default() -> Self {
        Self {
            menu_selection: MainMenuSelection::None,
            animal_selection: ZooAnimal::None,
            terrain_selection: WorldTerrain::None,
            rock_selection: WorldRock::None,
        }
    }
}
