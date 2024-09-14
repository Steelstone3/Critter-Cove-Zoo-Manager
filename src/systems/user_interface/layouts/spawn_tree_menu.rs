use bevy::{
    asset::AssetServer,
    color::Color,
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    ui::{node_bundles::NodeBundle, Display, GridTrack, PositionType, Style, Val},
};

use crate::{
    assets::images::{user_interface::tree_sub_menu::TreeSubMenu, world::trees::WorldTree},
    components::{constants::TILE_SIZE, user_interface::SubMenu},
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SubMenuEntityQuery,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{
        interactions::main_menu_selection::MainMenuSelection,
        styles::{create_tree_button_bundle, create_tree_button_icon},
    },
};

pub fn spawn_tree_menu(
    selected_item: Res<SelectedMenuItem>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SubMenuEntityQuery>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if selected_item.menu_selection == MainMenuSelection::Trees {
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
                        grid_template_rows: vec![
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                        ],
                        width: Val::Px(TILE_SIZE * 1.5 * 4.0),
                        height: Val::Px(TILE_SIZE * 1.5 * 8.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(64.0),
                        top: Val::Percent(0.0),
                        ..Default::default()
                    },
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..Default::default()
                })
                .insert(SubMenu)
                // Bush_1
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::Bush1))
                        .with_children(|parent| {
                            parent
                                .spawn(create_tree_button_icon(&asset_server, TreeSubMenu::Bush1));
                        });
                })
                // Bush_2
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::Bush2))
                        .with_children(|parent| {
                            parent
                                .spawn(create_tree_button_icon(&asset_server, TreeSubMenu::Bush2));
                        });
                })
                // Tall_Grass_1
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::TallGrass1))
                        .with_children(|parent| {
                            parent.spawn(create_tree_button_icon(
                                &asset_server,
                                TreeSubMenu::TallGrass1,
                            ));
                        });
                })
                // Tall_Grass_2
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::TallGrass2))
                        .with_children(|parent| {
                            parent.spawn(create_tree_button_icon(
                                &asset_server,
                                TreeSubMenu::TallGrass2,
                            ));
                        });
                })
                // Tall_Grass_3
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::TallGrass3))
                        .with_children(|parent| {
                            parent.spawn(create_tree_button_icon(
                                &asset_server,
                                TreeSubMenu::TallGrass3,
                            ));
                        });
                })
                // Tree_1
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::Tree1))
                        .with_children(|parent| {
                            parent
                                .spawn(create_tree_button_icon(&asset_server, TreeSubMenu::Tree1));
                        });
                })
                // Tree_2
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::Tree2))
                        .with_children(|parent| {
                            parent
                                .spawn(create_tree_button_icon(&asset_server, TreeSubMenu::Tree2));
                        });
                })
                // Tree_3
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::Tree3))
                        .with_children(|parent| {
                            parent
                                .spawn(create_tree_button_icon(&asset_server, TreeSubMenu::Tree3));
                        });
                })
                // Tree_4
                .with_children(|parent| {
                    parent
                        .spawn(create_tree_button_bundle(WorldTree::Tree4))
                        .with_children(|parent| {
                            parent
                                .spawn(create_tree_button_icon(&asset_server, TreeSubMenu::Tree4));
                        });
                });
        }
    }
}
