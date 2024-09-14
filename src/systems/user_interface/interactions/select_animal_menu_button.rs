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
    queries::user_interface_queries::{ButtonFilters, SelectAnimalMenuButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn select_animal_menu_button(
    mut select_animal_menu_button_queries: Query<SelectAnimalMenuButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_animal_menu_button_query) =
        select_animal_menu_button_queries.get_single_mut()
    else {
        return;
    };

    match *select_animal_menu_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Animal");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Animals;

            *select_animal_menu_button_query.border_color = Color::srgb(255.0, 238.0, 88.0).into();

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Animal");

            *select_animal_menu_button_query.border_color = Color::srgb(255.0, 238.0, 88.0).into();
        }
        Interaction::None => {
            *select_animal_menu_button_query.border_color = Color::srgb(189.0, 189.0, 189.0).into();
        }
    }
}
