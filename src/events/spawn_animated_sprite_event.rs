use bevy::ecs::event::Event;

use super::spawn_sprite_event::SpawnSpriteEvent;

#[derive(Event)]
pub struct SpawnAnimatedSpriteEvent {
    pub tile_size: u32,
    pub tile_columns: u32,
    pub frame_timing: f32,
    pub frame_count: usize,
    pub spawn_sprite_event: SpawnSpriteEvent,
}
