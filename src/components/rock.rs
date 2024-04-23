use crate::assets::images::world::rocks::WorldRock;
use bevy::{ecs::component::Component, math::Vec2};

use super::constants::TILE_SIZE;

#[derive(Component)]
pub struct Rock {
    pub sprite_path: WorldRock,
    pub size: Vec2,
    pub z_index: f32,
}

impl Rock {
    pub fn new(sprite_path: WorldRock) -> Self {
        Self {
            sprite_path,
            size: Vec2 {
                x: TILE_SIZE / 1.5,
                y: TILE_SIZE / 1.5,
            },
            z_index: 4.0,
        }
    }
}
