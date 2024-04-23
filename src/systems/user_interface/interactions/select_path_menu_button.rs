use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectPathMenuButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};
use bevy::{
    ecs::{
        event::EventWriter,
        system::{Query, ResMut},
    },
    render::color::Color,
    ui::Interaction,
    utils::tracing,
};

pub fn select_path_menu_button(
    mut select_path_menu_button_queries: Query<SelectPathMenuButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_path_menu_button_query) = select_path_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_path_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Path");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Paths;

            *select_path_menu_button_query.border_color = Color::YELLOW.into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Path");

            *select_path_menu_button_query.border_color = Color::YELLOW.into();
        }
        Interaction::None => {
            *select_path_menu_button_query.border_color = Color::DARK_GRAY.into();
        }
    }
}
