use bevy::{
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
    hierarchy::DespawnRecursiveExt,
};

use crate::{
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SubMenuEntityQuery,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn spawn_no_sub_menu(
    selected_item: Res<SelectedMenuItem>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SubMenuEntityQuery>,
    mut commands: Commands,
) {
    if selected_item.menu_selection != MainMenuSelection::None {
        return;
    }

    for _ in user_interface_events.read() {
        if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
            commands.entity(sub_menu_query.entity).despawn_recursive();
        }
    }
}
