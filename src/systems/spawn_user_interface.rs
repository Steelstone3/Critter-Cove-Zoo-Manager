use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    render::{color::Color, texture::Image},
    ui::{node_bundles::NodeBundle, Display, GridTrack, PositionType, Style, UiImage, Val},
};

use crate::{assets::images::user_interface::UserInterface, components::constants::TILE_SIZE};

pub fn spawn_user_interface(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            let texture: Handle<Image> = asset_server.load(UserInterface::IconAnimals.to_string());
            parent.spawn((icon_node_bundle(), UiImage::new(texture)));

            let texture: Handle<Image> = asset_server.load(UserInterface::IconAnimals.to_string());
            parent.spawn((icon_node_bundle(), UiImage::new(texture)));

            let texture: Handle<Image> = asset_server.load(UserInterface::IconAnimals.to_string());
            parent.spawn((icon_node_bundle(), UiImage::new(texture)));

            let texture: Handle<Image> = asset_server.load(UserInterface::IconAnimals.to_string());
            parent.spawn((icon_node_bundle(), UiImage::new(texture)));

            let texture: Handle<Image> = asset_server.load(UserInterface::IconAnimals.to_string());
            parent.spawn((icon_node_bundle(), UiImage::new(texture)));

            let texture: Handle<Image> = asset_server.load(UserInterface::IconAnimals.to_string());
            parent.spawn((icon_node_bundle(), UiImage::new(texture)));
        });
}

fn icon_node_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        // a `NodeBundle` is transparent by default, so to see the image we have to its color to `WHITE`
        background_color: Color::WHITE.into(),
        ..Default::default()
    }
}
