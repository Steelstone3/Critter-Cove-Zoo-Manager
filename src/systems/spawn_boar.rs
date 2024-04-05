use bevy::{ecs::event::EventWriter, math::Vec3, transform::components::Transform};

use crate::{
    components::animal::Animal,
    events::{
        spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sprite_event::SpawnSpriteEvent,
    },
};

pub fn spawn_boar(mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>) {
    let animal = Animal::new_boar();

    spawn_animated_sprite_event.send(SpawnAnimatedSpriteEvent {
        frame_timing: 0.25,
        frame_count: animal.frame_count,
        tile_size: animal.tile_size,
        tile_columns: animal.frame_count,
        spawn_sprite_event: SpawnSpriteEvent {
            sprite_path: animal.sprite_path.to_string(),
            size: animal.size,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
        },
    });
}
