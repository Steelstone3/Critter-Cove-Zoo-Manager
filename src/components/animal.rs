use super::constants::TILE_SIZE;
use crate::assets::images::animal::ZooAnimal;
use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Animal {
    pub sprite_path: ZooAnimal,
    pub frame_timing: f32,
    pub frame_count: usize,
    pub tile_size: f32,
    pub size: Vec2,
}

impl Animal {
    pub fn new(sprite_path: ZooAnimal) -> Self {
        Self {
            sprite_path,
            frame_timing: 0.1,
            frame_count: 4,
            tile_size: TILE_SIZE / 2.0,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
        }
    }
}
