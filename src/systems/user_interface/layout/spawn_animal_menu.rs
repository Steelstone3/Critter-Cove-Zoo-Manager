use bevy::{
    asset::AssetServer,
    ecs::{
        event::EventReader,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    render::color::Color,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle},
        Display, GridTrack, PositionType, Style, UiImage, UiRect, Val,
    },
};

use crate::{
    assets::images::{animal::ZooAnimal, user_interface::animal_sub_menu::AnimalSubMenu},
    components::{
        constants::TILE_SIZE,
        user_interface::{SelectAnimalButton, SubMenu},
    },
    events::user_interface_event::UserInterfaceEvent,
    queries::user_interface_queries::SubMenuEntityQuery,
    resources::selected_item::SelectedMenuItem,
    systems::user_interface::interactions::main_menu_selection::MainMenuSelection,
};

pub fn spawn_animal_menu(
    selected_item: Res<SelectedMenuItem>,
    mut user_interface_events: EventReader<UserInterfaceEvent>,
    sub_menu_queries: Query<SubMenuEntityQuery>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if selected_item.menu_selection != MainMenuSelection::Animals {
        return;
    }

    for _ in user_interface_events.read() {
        if let Ok(sub_menu_query) = sub_menu_queries.get_single() {
            commands.entity(sub_menu_query.entity).despawn_recursive();
        }
    }

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
        .with_children(|parent| {
            let style = Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),

                ..Default::default()
            };
            // Moose
            parent
                .spawn(create_button_bundle(style, ZooAnimal::Moose))
                .with_children(|parent| {
                    parent.spawn(create_button_icon(asset_server, AnimalSubMenu::MooseIcon));
                });
        });
}

fn create_button_bundle(style: Style, animal: ZooAnimal) -> (ButtonBundle, SelectAnimalButton) {
    (
        ButtonBundle {
            style,
            border_color: Color::DARK_GRAY.into(),
            ..Default::default()
        },
        SelectAnimalButton { animal },
    )
}

fn create_button_icon(
    asset_server: Res<AssetServer>,
    animal_sub_menu: AnimalSubMenu,
) -> ImageBundle {
    ImageBundle {
        image: UiImage::new(asset_server.load(animal_sub_menu.to_string())),
        background_color: Color::WHITE.into(),
        ..Default::default()
    }
}
