use bevy::{
    color::Color,
    ecs::{
        event::EventWriter,
        system::{Query, ResMut},
    },
    ui::Interaction,
    utils::tracing,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectFenceMenuButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{interactions::main_menu_selection::MainMenuSelection, styles::{GREY, YELLOW}},
};

pub fn select_fence_menu_button(
    mut select_fence_menu_button_queries: Query<SelectFenceMenuButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_fence_menu_button_query) = select_fence_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_fence_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Fence");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Fences;

            *select_fence_menu_button_query.border_color = YELLOW.into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Fence");

            *select_fence_menu_button_query.border_color = YELLOW.into();
        }
        Interaction::None => {
            *select_fence_menu_button_query.border_color = GREY.into();
        }
    }
}
