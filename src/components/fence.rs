use super::constants::TILE_SIZE;
use crate::assets::images::world::fences::WorldFence;
use bevy::{ecs::component::Component, math::Vec2};

const TERRAIN_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Fence {
    pub sprite_path: WorldFence,
    pub size: Vec2,
    pub z_index: f32,
}

impl Fence {
    pub fn new(sprite_path: WorldFence) -> Self {
        Self {
            sprite_path,
            size: TERRAIN_SIZE,
            z_index: 3.0,
        }
    }
}
