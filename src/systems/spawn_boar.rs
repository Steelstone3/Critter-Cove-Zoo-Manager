use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Commands, Res, ResMut},
    math::{Vec2, Vec3},
    sprite::{Sprite, SpriteSheetBundle, TextureAtlas, TextureAtlasLayout},
    transform::components::Transform,
};

use crate::components::{animal::Animal, constants::TILE_SIZE};

pub fn spawn_boar(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let animal = Animal::new_boar();
    let texture = asset_server.load(animal.source.to_string());
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
        4,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        SpriteSheetBundle {
            sprite: Sprite {
                custom_size: Some(animal.size),
                ..Default::default()
            },
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        animal.animation_timer,
    ));
}
