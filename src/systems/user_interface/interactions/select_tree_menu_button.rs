use bevy::{
    ecs::{
        event::EventWriter,
        system::{Query, ResMut},
    },
    ui::Interaction,
    utils::tracing,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectTreeMenuButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{
        interactions::main_menu_selection::MainMenuSelection,
        styles::{GREY, YELLOW},
    },
};

pub fn select_tree_menu_button(
    mut select_tree_menu_button_queries: Query<SelectTreeMenuButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_tree_menu_button_query) = select_tree_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_tree_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Tree");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Trees;

            *select_tree_menu_button_query.border_color = YELLOW.into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Tree");

            *select_tree_menu_button_query.border_color = YELLOW.into();
        }
        Interaction::None => {
            *select_tree_menu_button_query.border_color = GREY.into();
        }
    }
}
