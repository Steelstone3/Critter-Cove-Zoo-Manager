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
    queries::user_interface_queries::{ButtonFilters, SelectFenceButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{interactions::main_menu_selection::MainMenuSelection, styles::GREY},
};

pub fn select_fence_button(
    mut select_fence_button_queries: Query<SelectFenceButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_fence_button_query) = select_fence_button_queries.get_single_mut() else {
        return;
    };

    match *select_fence_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Fence");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Fences;
            selected_item.fence_selection = select_fence_button_query.selected_fence_button.fence;

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Fence");
        }
        Interaction::None => {
            *select_fence_button_query.border_color = GREY.into();
        }
    }
}
