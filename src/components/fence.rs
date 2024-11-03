use super::constants::TILE_SIZE;
use crate::assets::images::world::fence_sprites::FenceSprite;
use bevy::{ecs::component::Component, math::Vec2};

const TERRAIN_SIZE: Vec2 = Vec2::new(TILE_SIZE, TILE_SIZE);

#[derive(Component, Clone, Copy)]
pub struct Fence {
    pub sprite_path: FenceSprite,
    pub size: Vec2,
    pub z_index: f32,
}

impl Fence {
    pub fn new(sprite_path: FenceSprite) -> Self {
        Self {
            sprite_path,
            size: TERRAIN_SIZE,
            z_index: 3.0,
        }
    }
}
