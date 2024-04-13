use bevy::{
    ecs::{
        event::EventReader,
        query::Changed,
        system::{Query, ResMut},
    },
    input::{mouse::{MouseButton, MouseButtonInput}, ButtonState},
    ui::Interaction, utils::tracing,
};

use crate::{
    assets::images::{animal::ZooAnimal, world::terrain::WorldTerrain},
    components::user_interface::SelectAnimalButton,
    resources::selected_item::{MainMenuSelection, SelectedMenuItem},
};

pub fn select_animal_button(
    select_animal_button_queries: Query<(&SelectAnimalButton, &Interaction), Changed<Interaction>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut selected_item: ResMut<SelectedMenuItem>,
) {
    let Ok(select_animal_button_query) = select_animal_button_queries.get_single() else {
        return;
    };

    match *select_animal_button_query.1 {
        Interaction::Pressed => {
            tracing::info!("Pressed");

            for mouse_button_event in mouse_button_input_events.read() {
                if mouse_button_event.button != MouseButton::Left {
                    return;
                }
                if mouse_button_event.state != ButtonState::Pressed {
                    return;
                }

                selected_item.menu_selection = MainMenuSelection::Animals;
                selected_item.animal_selection = ZooAnimal::Chicken;
                selected_item.terrain_selection = WorldTerrain::None;
            }
        }
        Interaction::Hovered => {
            tracing::info!("Hovered");

            selected_item.menu_selection = MainMenuSelection::Animals;
        }
        Interaction::None => {
            selected_item.menu_selection = MainMenuSelection::None;
        }
    }
}
