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
    queries::user_interface_queries::{ButtonFilters, SelectTerrainButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{interactions::main_menu_selection::MainMenuSelection, styles::GREY},
};

pub fn select_terrain_button(
    mut select_terrain_button_queries: Query<SelectTerrainButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_terrain_button_query) = select_terrain_button_queries.get_single_mut() else {
        return;
    };

    match *select_terrain_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Terrain");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Terrain;
            selected_item.terrain_selection =
                select_terrain_button_query.selected_terrain_button.terrain;

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Terrain");
        }
        Interaction::None => {
            *select_terrain_button_query.border_color = GREY.into();
        }
    }
}
