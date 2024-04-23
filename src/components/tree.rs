use crate::assets::images::world::tree::WorldTree;
use bevy::{ecs::component::Component, math::Vec2};

use super::constants::TILE_SIZE;

#[derive(Component)]
pub struct Tree {
    pub sprite_path: WorldTree,
    pub size: Vec2,
    pub z_index: f32,
}

impl Tree {
    pub fn new_128(sprite_path: WorldTree) -> Self {
        Self {
            sprite_path,
            size: Vec2 {
                x: TILE_SIZE * 4.0,
                y: TILE_SIZE * 4.0,
            },
            z_index: 4.0,
        }
    }
    pub fn new_32(sprite_path: WorldTree) -> Self {
        Self {
            sprite_path,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            z_index: 4.0,
        }
    }
}
