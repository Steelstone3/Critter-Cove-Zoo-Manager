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
    assets::images::{
        animal::ZooAnimal,
        world::{terrains::WorldTerrain, trees::WorldTree},
    },
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectRockButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn select_rock_button(
    mut select_rock_button_queries: Query<SelectRockButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_rock_button_query) = select_rock_button_queries.get_single_mut() else {
        return;
    };

    match *select_rock_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Rock");

            selected_item.menu_selection = MainMenuSelection::Rocks;
            selected_item.rock_selection = select_rock_button_query.selected_rock_button.rock;
            selected_item.animal_selection = ZooAnimal::None;
            selected_item.terrain_selection = WorldTerrain::None;
            selected_item.tree_selection = WorldTree::None;

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Rock");
        }
        Interaction::None => {
            *select_rock_button_query.border_color = Color::DARK_GRAY.into();
        }
    }
}
