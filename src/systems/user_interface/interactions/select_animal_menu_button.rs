use bevy::{
    ecs::{
        event::EventWriter,
        query::Changed,
        system::{Query, ResMut},
    },
    ui::Interaction,
    utils::tracing,
};

use crate::{
    assets::images::{animal::ZooAnimal, world::terrain::WorldTerrain},
    components::user_interface::SelectAnimalMenuButton,
    events::user_interface_event::UserInterfaceEvent,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn select_animal_menu_button(
    // TODO Create a query
    select_animal_menu_button_queries: Query<
        (&SelectAnimalMenuButton, &Interaction),
        Changed<Interaction>,
    >,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(select_animal_menu_button_query) = select_animal_menu_button_queries.get_single() else {
        return;
    };

    match *select_animal_menu_button_query.1 {
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
