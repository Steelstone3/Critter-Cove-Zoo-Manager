use crate::assets::images::{
    animal::ZooAnimal,
    world::{
        fences::WorldFence, paths::WorldPath, rocks::WorldRock, terrains::WorldTerrain,
        trees::WorldTree,
    },
};
use bevy::ecs::component::Component;

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

#[derive(Component)]
pub struct SelectFenceMenuButton;

#[derive(Component)]
pub struct SelectFenceButton {
    pub fence: WorldFence,
}

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

#[derive(Component)]
pub struct SelectPathMenuButton;

#[derive(Component)]
pub struct SelectPathButton {
    pub path: WorldPath,
}
