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
    queries::user_interface_queries::{ButtonFilters, SelectAnimalButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn select_animal_button(
    mut select_animal_button_queries: Query<SelectAnimalButtonQuery, ButtonFilters>,
    mut selected_item: ResMut<SelectedMenuItem>,
    mut user_interface_event: EventWriter<UserInterfaceEvent>,
) {
    let Ok(mut select_animal_button_query) = select_animal_button_queries.get_single_mut() else {
        return;
    };

    match *select_animal_button_query.interaction {
        Interaction::Pressed => {
            tracing::info!("Pressed Animal");

            SelectedMenuItem::reset(&mut selected_item);
            selected_item.menu_selection = MainMenuSelection::Animals;
            selected_item.animal_selection =
                select_animal_button_query.selected_animal_button.animal;

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Animal");
        }
        Interaction::None => {
            // Dark Grey
            *select_animal_button_query.border_color = Color::srgb(189.0, 189.0, 189.0).into();
        }
    }
}
