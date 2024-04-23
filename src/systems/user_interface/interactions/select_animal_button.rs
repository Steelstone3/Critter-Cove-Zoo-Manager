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
    assets::images::world::{rocks::WorldRock, terrains::WorldTerrain, trees::WorldTree},
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

            selected_item.menu_selection = MainMenuSelection::Animals;
            selected_item.animal_selection =
                select_animal_button_query.selected_animal_button.animal;
            selected_item.terrain_selection = WorldTerrain::None;
            selected_item.rock_selection = WorldRock::None;
            selected_item.tree_selection = WorldTree::None;

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered Animal");
        }
        Interaction::None => {
            *select_animal_button_query.border_color = Color::DARK_GRAY.into();
        }
    }
}
