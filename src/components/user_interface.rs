use crate::assets::images::{
    animal::ZooAnimal,
    world::{rocks::WorldRock, tree::WorldTree},
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
pub struct SelectRockMenuButton;

#[derive(Component)]
pub struct SelectRockButton {
    pub rock: WorldRock,
}

#[derive(Component)]
pub struct SelectTreeMenuButton;

#[derive(Component)]
pub struct SelectTreeButton {
    pub tree: WorldTree,
}

// TODO implement
// #[derive(Component)]
// pub struct SelectTerrainMenuButton;
