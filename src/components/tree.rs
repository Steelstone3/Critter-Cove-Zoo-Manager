use crate::assets::images::world::tree_sprites::TreeSprite;
use bevy::{ecs::component::Component, math::Vec2};

use super::constants::TILE_SIZE;

#[derive(Component)]
pub struct Tree {
    pub sprite_path: TreeSprite,
    pub size: Vec2,
    pub z_index: f32,
}

impl Tree {
    pub fn new_128(sprite_path: TreeSprite) -> Self {
        Self {
            sprite_path,
            size: Vec2 {
                x: TILE_SIZE * 4.0,
                y: TILE_SIZE * 4.0,
            },
            z_index: 6.0,
        }
    }
    pub fn new_32(sprite_path: TreeSprite) -> Self {
        Self {
            sprite_path,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
            z_index: 6.0,
        }
    }
}
