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
    assets::images::{user_interface::rock_sub_menu::RockSubMenu, world::rocks::WorldRock},
    components::{constants::TILE_SIZE, user_interface::SubMenu},
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SubMenuEntityQuery,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{
        interactions::main_menu_selection::MainMenuSelection,
        styles::{create_rock_button_bundle, create_rock_button_icon},
    },
};

pub fn spawn_rock_menu(
    selected_item: Res<SelectedMenuItem>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SubMenuEntityQuery>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if selected_item.menu_selection == MainMenuSelection::Rocks {
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
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..Default::default()
                })
                .insert(SubMenu)
                // Ice Rock_1
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::IceRock1))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::IceRock1,
                            ));
                        });
                })
                // Ice Rock_2
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::IceRock2))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::IceRock2,
                            ));
                        });
                })
                // Ice Rock_3
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::IceRock3))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::IceRock3,
                            ));
                        });
                })
                // Ice Rock_4
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::IceRock4))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::IceRock4,
                            ));
                        });
                })
                // Ice Rock_5
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::IceRock5))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::IceRock5,
                            ));
                        });
                })
                // Ice Rock_6
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::IceRock6))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::IceRock6,
                            ));
                        });
                })
                // Rock_1
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock1))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock1));
                        });
                })
                // Rock_2
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock2))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock2));
                        });
                })
                // Rock_3
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock3))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock3));
                        });
                })
                // Rock_4
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock4))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock4));
                        });
                })
                // Rock_5
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock5))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock5));
                        });
                })
                // Rock_6
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock6))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock6));
                        });
                })
                // Rock_7
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock7))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock7));
                        });
                })
                // Rock_8
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock8))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock8));
                        });
                })
                // Rock_9
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock9))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock9));
                        });
                })
                // Rock_10
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock10))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock10));
                        });
                })
                // Rock_11
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock11))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock11));
                        });
                })
                // Rock_12
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock12))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock12));
                        });
                })
                // Rock_13
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock13))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock13));
                        });
                })
                // Rock_14
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock14))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock14));
                        });
                })
                // Rock_15
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock15))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock15));
                        });
                })
                // Rock_16
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock16))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock16));
                        });
                })
                // Rock_17
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::Rock17))
                        .with_children(|parent| {
                            parent
                                .spawn(create_rock_button_icon(&asset_server, RockSubMenu::Rock17));
                        });
                })
                // Water Rock_1
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::WaterRock1))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::WaterRock1,
                            ));
                        });
                })
                // Water Rock_2
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::WaterRock2))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::WaterRock2,
                            ));
                        });
                })
                // Water Rock_3
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::WaterRock3))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::WaterRock3,
                            ));
                        });
                })
                // Water Rock_4
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::WaterRock4))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::WaterRock4,
                            ));
                        });
                })
                // Water Rock_5
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::WaterRock5))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::WaterRock5,
                            ));
                        });
                })
                // Water Rock_6
                .with_children(|parent| {
                    parent
                        .spawn(create_rock_button_bundle(WorldRock::WaterRock6))
                        .with_children(|parent| {
                            parent.spawn(create_rock_button_icon(
                                &asset_server,
                                RockSubMenu::WaterRock6,
                            ));
                        });
                });
        }
    }
}
