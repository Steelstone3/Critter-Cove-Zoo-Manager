use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        event::EventReader,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::{color::Color, texture::Image},
    ui::{
        node_bundles::NodeBundle, widget::Button, Display, GridTrack, PositionType, Style, UiImage,
        Val,
    },
};

use crate::{
    assets::images::user_interface::MainMenuUserInterface, components::constants::TILE_SIZE,
    events::user_interface_event::UserInterfaceEvent, resources::selected_item::SelectedMenuItem,
};

pub fn update_user_interface(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    _selected_menu_item: Res<SelectedMenuItem>,
    mut user_interface_event: EventReader<UserInterfaceEvent>,
) {
    for _ in user_interface_event.read() {
        // if let Ok(weapon_selection_parent) = menu_selection_queries.get_single() {
        //     commands
        //         .entity(weapon_selection_parent.entity)
        //         .despawn_recursive();
        // }

        commands
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::flex(1.0)],
                    grid_template_rows: vec![
                        GridTrack::flex(1.0),
                        GridTrack::flex(1.0),
                        GridTrack::flex(1.0),
                        GridTrack::flex(1.0),
                        GridTrack::flex(1.0),
                        GridTrack::flex(1.0),
                    ],
                    width: Val::Px(TILE_SIZE * 2.0),
                    height: Val::Px(TILE_SIZE * 6.0 * 2.0),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.0),
                    top: Val::Percent(0.0),
                    ..Default::default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
                ..Default::default()
            })
            .with_children(|parent| {
                let texture: Handle<Image> =
                    asset_server.load(MainMenuUserInterface::SelectedIconAnimals.to_string());
                parent.spawn((icon_node_bundle(), Button, UiImage::new(texture)));

                let texture: Handle<Image> =
                    asset_server.load(MainMenuUserInterface::IconAnimals.to_string());
                parent.spawn((icon_node_bundle(), Button, UiImage::new(texture)));

                let texture: Handle<Image> =
                    asset_server.load(MainMenuUserInterface::IconAnimals.to_string());
                parent.spawn((icon_node_bundle(), Button, UiImage::new(texture)));

                let texture: Handle<Image> =
                    asset_server.load(MainMenuUserInterface::IconAnimals.to_string());
                parent.spawn((icon_node_bundle(), Button, UiImage::new(texture)));

                let texture: Handle<Image> =
                    asset_server.load(MainMenuUserInterface::IconAnimals.to_string());
                parent.spawn((icon_node_bundle(), Button, UiImage::new(texture)));

                let texture: Handle<Image> =
                    asset_server.load(MainMenuUserInterface::IconAnimals.to_string());
                parent.spawn((icon_node_bundle(), Button, UiImage::new(texture)));
            });
    }

    fn icon_node_bundle() -> NodeBundle {
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: Color::WHITE.into(),
            ..Default::default()
        }
    }
}
