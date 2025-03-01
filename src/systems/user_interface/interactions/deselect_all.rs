use crate::resources::selected_item::SelectedMenuItem;
use bevy::{
    ecs::system::ResMut,
    input::{keyboard::KeyCode, ButtonInput},
};

pub fn deselect_all(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut selected_item: ResMut<SelectedMenuItem>,
) {
    if input.clear_just_pressed(KeyCode::Escape) {
        *selected_item = SelectedMenuItem::default();
    }
}
