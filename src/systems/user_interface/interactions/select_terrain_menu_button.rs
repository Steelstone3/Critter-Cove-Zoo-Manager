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
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectTerrainMenuButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn select_terrain_menu_button(
    mut select_terrain_menu_button_queries: Query<SelectTerrainMenuButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_rock_menu_button_query) = select_terrain_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_rock_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Terrain");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Terrain;

            *select_rock_menu_button_query.border_color = Color::YELLOW.into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Terrain");

            *select_rock_menu_button_query.border_color = Color::YELLOW.into();
        }
        Interaction::None => {
            *select_rock_menu_button_query.border_color = Color::DARK_GRAY.into();
        }
    }
}
