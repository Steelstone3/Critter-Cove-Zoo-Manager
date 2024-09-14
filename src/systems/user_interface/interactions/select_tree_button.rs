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
    queries::user_interface_queries::{ButtonFilters, SelectTreeButtonQuery},
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
            tracing::info!("Pressed Tree");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Trees;
            selected_item.tree_selection = select_tree_button_query.selected_tree_button.tree;

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Tree");
        }
        Interaction::None => {
            *select_tree_button_query.border_color = Color::srgb(189.0, 189.0, 189.0).into();
        }
    }
}
