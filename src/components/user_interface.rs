use crate::assets::images::{
    animal::ZooAnimal,
    world::{rocks::WorldRock, terrains::WorldTerrain, tree::WorldTree},
};
use bevy::ecs::component::Component;

use super::terrain::Terrain;

#[derive(Component)]
pub struct SelectionMenu;

#[derive(Component)]
pub struct SubMenu;

#[derive(Component)]
pub struct SelectAnimalMenuButton;

#[derive(Component)]
pub struct SelectAnimalButton {
    pub animal: ZooAnimal,
}

// TODO implement
// #[derive(Component)]
// pub struct SelectFenceMenuButton;

// TODO implement
// #[derive(Component)]
// pub struct SelectFenceButton {
//     pub fence: Fence,
// }

#[derive(Component)]
pub struct SelectTerrainMenuButton;

#[derive(Component)]
pub struct SelectTerrainButton {
    pub terrain: WorldTerrain,
}

#[derive(Component)]
pub struct SelectTreeMenuButton;

#[derive(Component)]
pub struct SelectTreeButton {
    pub tree: WorldTree,
}

#[derive(Component)]
pub struct SelectRockMenuButton;

#[derive(Component)]
pub struct SelectRockButton {
    pub rock: WorldRock,
}

// TODO implement
// #[derive(Component)]
// pub struct SelectShelterMenuButton;

// TODO implement
// #[derive(Component)]
// pub struct SelectShelterButton {
//     pub shelter: Shelter,
// }

// TODO implement
// #[derive(Component)]
// pub struct SelectPathMenuButton;

// TODO implement
// #[derive(Component)]
// pub struct SelectPathButton {
//     pub path: Path,
// }
