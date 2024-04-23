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
        user_interface::terrain_sub_menu::TerrainSubMenu, world::terrains::WorldTerrain,
    },
    components::{constants::TILE_SIZE, user_interface::SubMenu},
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SubMenuEntityQuery,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{
        interactions::main_menu_selection::MainMenuSelection,
        styles::{create_terrain_button_bundle, create_terrain_button_icon},
    },
};

pub fn spawn_terrain_menu(
    selected_item: Res<SelectedMenuItem>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SubMenuEntityQuery>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if selected_item.menu_selection == MainMenuSelection::Terrain {
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
                // Dark_Grass_1
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass1))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass1,
                            ));
                        });
                })
                // Dark_Grass_2
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass2))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass2,
                            ));
                        });
                })
                // Dark_Grass_3
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass3))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass3,
                            ));
                        });
                })
                // Dark_Grass_4
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass4))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass4,
                            ));
                        });
                })
                // Dark_Grass_5
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass5))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass5,
                            ));
                        });
                })
                // Dark_Grass_6
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass6))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass6,
                            ));
                        });
                })
                // Dark_Grass_7
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass7))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass7,
                            ));
                        });
                })
                // Dark_Grass_8
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass8))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass8,
                            ));
                        });
                })
                // Dark_Grass_9
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::DarkGrass9))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::DarkGrass9,
                            ));
                        });
                })
                // Grass_1
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Grass1))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Grass1,
                            ));
                        });
                })
                // Grass_2
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Grass2))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Grass2,
                            ));
                        });
                })
                // Grass_3
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Grass3))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Grass3,
                            ));
                        });
                })
                // Grass_4
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Grass4))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Grass4,
                            ));
                        });
                })
                // Grass_5
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Grass5))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Grass5,
                            ));
                        });
                })
                // Grass_6
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Grass6))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Grass6,
                            ));
                        });
                })
                // Light_Grass_1
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::LightGrass1))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::LightGrass1,
                            ));
                        });
                })
                // Light_Grass_2
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::LightGrass2))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::LightGrass2,
                            ));
                        });
                })
                // Light_Grass_3
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::LightGrass3))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::LightGrass3,
                            ));
                        });
                })
                // Light_Grass_4
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::LightGrass4))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::LightGrass4,
                            ));
                        });
                })
                // Light_Grass_5
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::LightGrass5))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::LightGrass5,
                            ));
                        });
                })
                // Light_Grass_6
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::LightGrass6))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::LightGrass6,
                            ));
                        });
                })
                // Savanah_1
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Savanah1))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Savanah1,
                            ));
                        });
                })
                // Savanah_2
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Savanah2))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Savanah2,
                            ));
                        });
                })
                // Savanah_3
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Savanah3))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Savanah3,
                            ));
                        });
                })
                // Savanah_4
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Savanah4))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Savanah4,
                            ));
                        });
                })
                // Very_Light_Grass_1
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::VeryLightGrass1))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::VeryLightGrass1,
                            ));
                        });
                })
                // Very_Light_Grass_2
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::VeryLightGrass2))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::VeryLightGrass2,
                            ));
                        });
                })
                // Very_Light_Grass_3
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::VeryLightGrass3))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::VeryLightGrass3,
                            ));
                        });
                })
                // Very_Light_Grass_4
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::VeryLightGrass4))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::VeryLightGrass4,
                            ));
                        });
                })
                // Very_Light_Grass_5
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::VeryLightGrass5))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::VeryLightGrass5,
                            ));
                        });
                })
                // Water_1
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water1))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water1,
                            ));
                        });
                })
                // Water_2
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water2))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water2,
                            ));
                        });
                })
                // Water_3
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water3))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water3,
                            ));
                        });
                })
                // Water_4
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water4))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water4,
                            ));
                        });
                })
                // Water_5
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water5))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water5,
                            ));
                        });
                })
                // Water_6
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water6))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water6,
                            ));
                        });
                })
                // Water_7
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water7))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water7,
                            ));
                        });
                })
                // Water_8
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water8))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water8,
                            ));
                        });
                })
                // Water_9
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water9))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water9,
                            ));
                        });
                })
                // Water_10
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water10))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water10,
                            ));
                        });
                })
                // Water_11
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water11))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water11,
                            ));
                        });
                })
                // Water_12
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water12))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water12,
                            ));
                        });
                })
                // Water_13
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water13))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water13,
                            ));
                        });
                })
                // Water_14
                .with_children(|parent| {
                    parent
                        .spawn(create_terrain_button_bundle(WorldTerrain::Water14))
                        .with_children(|parent| {
                            parent.spawn(create_terrain_button_icon(
                                &asset_server,
                                TerrainSubMenu::Water14,
                            ));
                        });
                });
        }
    }
}
