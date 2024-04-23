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
    assets::images::{
        animal::ZooAnimal,
        world::{rocks::WorldRock, terrains::WorldTerrain, tree::WorldTree},
    },
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::{ButtonFilters, SelectRockButtonQuery, SelectTerrainButtonQuery},
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
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
            tracing::info!("Pressed");

            selected_item.menu_selection = MainMenuSelection::Rocks;
            selected_item.terrain_selection = select_terrain_button_query.selected_terrain_button.terrain;
            selected_item.animal_selection = ZooAnimal::None;
            selected_item.rock_selection = WorldRock::None;
            selected_item.tree_selection = WorldTree::None;

            user_interface_event.send(UserInterfaceEvent {});
        }
        Interaction::Hovered => {
            tracing::info!("Hovered");
        }
        Interaction::None => {
            *select_terrain_button_query.border_color = Color::DARK_GRAY.into();
        }
    }
}
