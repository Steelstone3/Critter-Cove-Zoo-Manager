use crate::assets::images::{animal::ZooAnimal, world::rocks::WorldRock};
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

// TODO implement
// #[derive(Component)]
// pub struct SelectTerrainMenuButton;
