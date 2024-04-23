use super::constants::TILE_SIZE;
use crate::assets::images::world::paths::WorldPath;
use bevy::{ecs::component::Component, math::Vec2};

const TERRAIN_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Path {
    pub sprite_path: WorldPath,
    pub size: Vec2,
    pub z_index: f32,
}

impl Path {
    pub fn new(sprite_path: WorldPath) -> Self {
        Self {
            sprite_path,
            size: TERRAIN_SIZE,
            z_index: 2.0,
        }
    }
}
