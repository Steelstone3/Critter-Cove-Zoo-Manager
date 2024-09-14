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
    queries::user_interface_queries::{ButtonFilters, SelectRockMenuButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{
        interactions::main_menu_selection::MainMenuSelection,
        styles::{GREY, YELLOW},
    },
};

pub fn select_rock_menu_button(
    mut select_rock_menu_button_queries: Query<SelectRockMenuButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_rock_menu_button_query) = select_rock_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_rock_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Rock");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Rocks;

            *select_rock_menu_button_query.border_color = YELLOW.into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Rock");

            *select_rock_menu_button_query.border_color = YELLOW.into();
        }
        Interaction::None => {
            *select_rock_menu_button_query.border_color = GREY.into();
        }
    }
}
