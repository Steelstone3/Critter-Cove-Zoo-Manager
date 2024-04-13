// TODO Remove this once referenced

// use crate::{
//     assets::images::{animal::ZooAnimal, world::terrain::WorldTerrain},
//     events::user_interface_event::UserInterfaceEvent,
//     resources::selected_item::{MainMenuSelection, SelectedMenuItem},
// };
// use bevy::{
//     ecs::{
//         event::EventWriter,
//         system::{Res, ResMut},
//     },
//     input::{keyboard::KeyCode, ButtonInput},
// };

// pub fn menu_selection(
//     input: Res<ButtonInput<KeyCode>>,
//     mut selected_item: ResMut<SelectedMenuItem>,
//     mut user_interface_event: EventWriter<UserInterfaceEvent>,
// ) {
//     if input.pressed(KeyCode::Escape) {
//         selected_item.menu_selection = MainMenuSelection::None;
//         selected_item.animal_selection = ZooAnimal::None;
//         selected_item.terrain_selection = WorldTerrain::None;
//         user_interface_event.send(UserInterfaceEvent {});
//     }

//     if input.pressed(KeyCode::Numpad1) || input.pressed(KeyCode::Digit1) {
//         selected_item.menu_selection = MainMenuSelection::Animals;
//         // TODO change this to None and get it from the sub menu
//         selected_item.animal_selection = ZooAnimal::Chicken;
//         selected_item.terrain_selection = WorldTerrain::None;
//         user_interface_event.send(UserInterfaceEvent {});
//     }

//     if input.pressed(KeyCode::Numpad2) || input.pressed(KeyCode::Digit2) {
//         selected_item.menu_selection = MainMenuSelection::Fences;
//         selected_item.animal_selection = ZooAnimal::None;
//         selected_item.terrain_selection = WorldTerrain::None;
//         user_interface_event.send(UserInterfaceEvent {});
//     }

//     if input.pressed(KeyCode::Numpad3) || input.pressed(KeyCode::Digit3) {
//         selected_item.menu_selection = MainMenuSelection::Terrain;
//         selected_item.animal_selection = ZooAnimal::None;
//         selected_item.terrain_selection = WorldTerrain::None;
//         user_interface_event.send(UserInterfaceEvent {});
//     }

//     if input.pressed(KeyCode::Numpad4) || input.pressed(KeyCode::Digit4) {
//         selected_item.menu_selection = MainMenuSelection::Trees;
//         selected_item.animal_selection = ZooAnimal::None;
//         selected_item.terrain_selection = WorldTerrain::None;
//         user_interface_event.send(UserInterfaceEvent {});
//     }

//     if input.pressed(KeyCode::Numpad5) || input.pressed(KeyCode::Digit5) {
//         selected_item.menu_selection = MainMenuSelection::Rocks;
//         selected_item.animal_selection = ZooAnimal::None;
//         selected_item.terrain_selection = WorldTerrain::None;
//         user_interface_event.send(UserInterfaceEvent {});
//     }

//     if input.pressed(KeyCode::Numpad6) || input.pressed(KeyCode::Digit6) {
//         selected_item.menu_selection = MainMenuSelection::Shelters;
//         selected_item.animal_selection = ZooAnimal::None;
//         selected_item.terrain_selection = WorldTerrain::None;
//         user_interface_event.send(UserInterfaceEvent {});
//     }
// }
