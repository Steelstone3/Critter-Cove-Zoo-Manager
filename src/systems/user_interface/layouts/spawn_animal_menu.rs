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
    assets::images::{animal::ZooAnimal, user_interface::animal_sub_menu::AnimalSubMenu},
    components::{constants::TILE_SIZE, user_interface::SubMenu},
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SubMenuEntityQuery,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::{
        interactions::main_menu_selection::MainMenuSelection,
        styles::{create_animal_button_bundle, create_animal_button_icon},
    },
};

pub fn spawn_animal_menu(
    selected_item: Res<SelectedMenuItem>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SubMenuEntityQuery>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if selected_item.menu_selection == MainMenuSelection::Animals {
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
                        ],
                        grid_template_rows: vec![
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                            GridTrack::flex(1.0),
                        ],
                        width: Val::Px(TILE_SIZE * 2.0 * 3.0),
                        height: Val::Px(TILE_SIZE * 2.0 * 7.0),
                        position_type: PositionType::Absolute,
                        left: Val::Px(64.0),
                        top: Val::Percent(0.0),
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                    ..Default::default()
                })
                .insert(SubMenu)
                // Boar
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Boar))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Boar,
                            ));
                        });
                })
                // Chicken
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Chicken))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Chicken,
                            ));
                        });
                })
                // Cow
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Cow))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Cow,
                            ));
                        });
                })
                // Crab
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Crab))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Crab,
                            ));
                        });
                })
                // Dog
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Dog))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Dog,
                            ));
                        });
                })
                // Fox
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Fox))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Fox,
                            ));
                        });
                })
                // Frog
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Frog))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Frog,
                            ));
                        });
                })
                // Goat
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Goat))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Goat,
                            ));
                        });
                })
                // Goose
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Goose))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Goose,
                            ));
                        });
                })
                // Gorilla
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Gorilla))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Gorilla,
                            ));
                        });
                })
                // Monkey
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Monkey))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Monkey,
                            ));
                        });
                })
                // Moose
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Moose))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Moose,
                            ));
                        });
                })
                // Pig
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Pig))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Pig,
                            ));
                        });
                })
                // Porcupine
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Porcupine))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Porcupine,
                            ));
                        });
                })
                // Sheep
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Sheep))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Sheep,
                            ));
                        });
                })
                // Skunk
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Skunk))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Skunk,
                            ));
                        });
                })
                // Toad
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Toad))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Toad,
                            ));
                        });
                })
                // Turtle
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Turtle))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Turtle,
                            ));
                        });
                })
                // Wolf
                .with_children(|parent| {
                    parent
                        .spawn(create_animal_button_bundle(ZooAnimal::Wolf))
                        .with_children(|parent| {
                            parent.spawn(create_animal_button_icon(
                                &asset_server,
                                AnimalSubMenu::Wolf,
                            ));
                        });
                });
        }
    } else if selected_item.menu_selection == MainMenuSelection::None {
        // Remove UI
        if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
            commands.entity(sub_menu_query.entity).despawn_recursive();
        }
    }
}
