use bevy::{
    ecs::{
        event::EventWriter,
        system::{Query, ResMut},
    },
    ui::Interaction,
    utils::tracing,
};

use crate::{
    assets::images::{animal::ZooAnimal, world::terrain::WorldTerrain},
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectAnimalMenuButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn select_animal_menu_button(
    select_animal_menu_button_queries: Query<SelectAnimalMenuButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(select_animal_menu_button_query) = select_animal_menu_button_queries.get_single() else {
        return;
    };

    match *select_animal_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed");

            selected_item.menu_selection = MainMenuSelection::Animals;
            selected_item.animal_selection = ZooAnimal::None;
            selected_item.terrain_selection = WorldTerrain::None;
            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered");

            selected_item.menu_selection = MainMenuSelection::Animals;
            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::None => {
            selected_item.menu_selection = MainMenuSelection::None;
        }
    }
}