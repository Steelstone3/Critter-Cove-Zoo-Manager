use super::constants::TILE_SIZE;
use crate::assets::images::animal::ZooAnimal;
use bevy::{
    ecs::component::Component,
    math::{Quat, Vec2},
};

#[derive(Component)]
pub struct Animal {
    pub sprite_path: ZooAnimal,
    pub frame_timing: f32,
    pub frame_count: usize,
    pub tile_size: f32,
    pub size: Vec2,
    pub speed: f32,
    pub direction: Quat,
}

impl Animal {
    pub fn new_16(sprite_path: ZooAnimal) -> Self {
        Self {
            sprite_path,
            frame_timing: 0.25,
            frame_count: 4,
            tile_size: TILE_SIZE / 2.0,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            speed: 5.0,
            direction: Default::default(),
        }
    }
    pub fn new_32(sprite_path: ZooAnimal) -> Self {
        Self {
            sprite_path,
            frame_timing: 0.1,
            frame_count: 8,
            tile_size: TILE_SIZE,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            speed: 5.0,
            direction: Default::default(),
        }
    }
}
