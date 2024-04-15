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

pub fn despawn_sub_menus(
    selected_item: Res<SelectedMenuItem>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SubMenuEntityQuery>,
    mut commands: Commands,
) {
    for _ in user_interface_events.read() {
        if selected_item.menu_selection == MainMenuSelection::None {
            // Remove UI
            if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
                commands.entity(sub_menu_query.entity).despawn_recursive();
            }
        }
    }
}
