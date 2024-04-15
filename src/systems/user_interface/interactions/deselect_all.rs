use bevy::{
    ecs::{event::EventWriter, system::ResMut},
    input::{keyboard::KeyCode, ButtonInput},
};

use super::main_menu_selection::MainMenuSelection;
use crate::{
    assets::images::{animal::ZooAnimal, world::{rocks::WorldRock, terrains::WorldTerrain}},
    events::user_interface_event::UserInterfaceEvent,
    resources::selected_item::SelectedMenuItem,
};

pub fn deselect_all(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    if input.clear_just_pressed(KeyCode::Escape) {
        selected_item.menu_selection = MainMenuSelection::None;
        selected_item.animal_selection = ZooAnimal::None;
        selected_item.terrain_selection = WorldTerrain::None;
        selected_item.rock_selection = WorldRock::None;

        user_interface_event.send(UserInterfaceEvent {});
    }
}
