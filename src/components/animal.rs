use super::constants::{MAP_SIZE, TILE_SIZE};
use crate::{
    assets::images::animal_sprites::AnimalSprite,
    systems::controllers::random_generator::{generate_seed, random_value_f32},
};
use bevy::{
    ecs::component::Component,
    math::{Vec2, Vec3},
};

#[derive(Component)]
pub struct Animal {
    pub sprite_path: AnimalSprite,
    pub frame_timing: f32,
    pub frame_count: usize,
    pub tile_size: u32,
    pub size: Vec2,
    pub speed: f32,
    pub destination: Vec3,
    pub z_index: f32,
}

impl Animal {
    pub fn new_16(sprite_path: AnimalSprite) -> Self {
        Self {
            sprite_path,
            frame_timing: 0.25,
            frame_count: 4,
            tile_size: (TILE_SIZE / 2.0) as u32,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            speed: 20.0,
            destination: Vec3::new(
                random_value_f32(generate_seed(), -MAP_SIZE..MAP_SIZE),
                random_value_f32(generate_seed(), -MAP_SIZE..MAP_SIZE),
                1.0,
            ),
            z_index: 5.0,
        }
    }
    pub fn new_32(sprite_path: AnimalSprite) -> Self {
        Self {
            sprite_path,
            frame_timing: 0.1,
            frame_count: 8,
            tile_size: TILE_SIZE as u32,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            speed: 30.0,
            destination: Vec3::new(
                random_value_f32(generate_seed(), -MAP_SIZE..MAP_SIZE),
                random_value_f32(generate_seed(), -MAP_SIZE..MAP_SIZE),
                1.0,
            ),
            z_index: 5.0,
        }
    }
}
