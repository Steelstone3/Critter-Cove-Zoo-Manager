use bevy::{
    ecs::{
        event::EventWriter,
        system::{Query, ResMut},
    },
    render::color::Color,
    ui::Interaction,
    utils::tracing,
};

use crate::{
    assets::images::{animal::ZooAnimal, world::{rocks::WorldRock, terrains::WorldTerrain}},
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectRockButtonQuery, SelectTreeButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn select_tree_button(
    mut select_tree_button_queries: Query<SelectTreeButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_tree_button_query) = select_tree_button_queries.get_single_mut() else {
        return;
    };

    match *select_tree_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed");

            selected_item.menu_selection = MainMenuSelection::Trees;
            selected_item.tree_selection = select_tree_button_query.selected_tree_button.tree;
            selected_item.animal_selection = ZooAnimal::None;
            selected_item.terrain_selection = WorldTerrain::None;
            selected_item.rock_selection = WorldRock::None;

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered");
        }
        Interaction::None => {
            *select_tree_button_query.border_color = Color::DARK_GRAY.into();
        }
    }
}
