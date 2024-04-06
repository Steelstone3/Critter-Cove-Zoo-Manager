use bevy::ecs::system::Resource;

use crate::assets::images::{animal::ZooAnimal, world::terrain::WorldTerrain};

#[derive(Resource)]
pub struct SelectedItem {
    pub animal: ZooAnimal,
    pub terrain: WorldTerrain,
}
