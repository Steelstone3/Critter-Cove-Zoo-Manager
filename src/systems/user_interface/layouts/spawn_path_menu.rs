use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    render::color::Color,
    ui::{node_bundles::NodeBundle, Display, GridTrack, PositionType, Style, Val},
};

use crate::{
    assets::images::{
        user_interface::{fence_sub_menu::FenceSubMenu, path_sub_menu::PathSubMenu},
        world::{fences::WorldFence, paths::WorldPath},
    },
    components::{constants::TILE_SIZE, user_interface::SubMenu},
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SubMenuEntityQuery,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{
        interactions::main_menu_selection::MainMenuSelection,
        styles::{
            create_fence_button_bundle, create_fence_button_icon, create_path_button_bundle,
            create_path_button_icon,
        },
    },
};

pub fn spawn_path_menu(
    selected_item: Res<SelectedMenuItem>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SubMenuEntityQuery>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if selected_item.menu_selection == MainMenuSelection::Paths {
        for _ in user_interface_events.read() {
            // Remove UI
            if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
                commands.entity(sub_menu_query.entity).despawn_recursive();
            }

            // Spawn UI
            commands
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_template_columns: vec![
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                        ],
                        grid_template_rows: vec![GridTrack::flex(1.0), GridTrack::flex(1.0)],
                        width: Val::Px(TILE_SIZE * 1.5 * 4.0),
                        height: Val::Px(TILE_SIZE * 1.5 * 8.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(64.0),
                        top: Val::Percent(0.0),
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..Default::default()
                })
                .insert(SubMenu)
                // Path_1
                .with_children(|parent| {
                    parent
                        .spawn(create_path_button_bundle(WorldPath::Path1))
                        .with_children(|parent| {
                            parent
                                .spawn(create_path_button_icon(&asset_server, PathSubMenu::Path1));
                        });
                })
                // Path_2
                .with_children(|parent| {
                    parent
                        .spawn(create_path_button_bundle(WorldPath::Path2))
                        .with_children(|parent| {
                            parent
                                .spawn(create_path_button_icon(&asset_server, PathSubMenu::Path2));
                        });
                })
                // Path_3
                .with_children(|parent| {
                    parent
                        .spawn(create_path_button_bundle(WorldPath::Path3))
                        .with_children(|parent| {
                            parent
                                .spawn(create_path_button_icon(&asset_server, PathSubMenu::Path3));
                        });
                })
                // Path_4
                .with_children(|parent| {
                    parent
                        .spawn(create_path_button_bundle(WorldPath::Path4))
                        .with_children(|parent| {
                            parent
                                .spawn(create_path_button_icon(&asset_server, PathSubMenu::Path4));
                        });
                })
                // Path_5
                .with_children(|parent| {
                    parent
                        .spawn(create_path_button_bundle(WorldPath::Path5))
                        .with_children(|parent| {
                            parent
                                .spawn(create_path_button_icon(&asset_server, PathSubMenu::Path5));
                        });
                });
        }
    }
}
