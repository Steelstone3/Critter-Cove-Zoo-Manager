use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::Vec3,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use crate::components::animal::Animal;

pub fn spawn_animal(mut commands: Commands, asset_server: Res<AssetServer>) {
    let animal = Animal::new_boar();
    let texture = asset_server.load(animal.source.to_string());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(animal.size),
            ..Default::default()
        },
        texture,
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
