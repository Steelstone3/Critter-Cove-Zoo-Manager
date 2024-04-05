use super::constants::TILE_SIZE;
use crate::assets::images::animal::ZooAnimal;
use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub struct Animal {
    pub source: ZooAnimal,
    pub frame_timing: f32,
    pub frame_count: usize,
    pub size: Vec2,
}

impl Animal {
    pub fn new_boar() -> Self {
        Self {
            source: ZooAnimal::Boar,
            frame_timing: 0.1,
            frame_count: 4,
            size: Vec2 {
                x: TILE_SIZE,
                y: TILE_SIZE,
            },
        }
    }
}
