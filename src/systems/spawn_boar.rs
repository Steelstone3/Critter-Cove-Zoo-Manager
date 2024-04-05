use bevy::{ecs::event::EventWriter, math::Vec3, transform::components::Transform};

use crate::{
    components::{animal::Animal, constants::TILE_SIZE},
    events::{
        spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sprite_event::SpawnSpriteEvent,
    },
};

pub fn spawn_boar(mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>) {
    let animal = Animal::new_boar();

    spawn_animated_sprite_event.send(SpawnAnimatedSpriteEvent {
        frame_timing: 0.1,
        frame_count: 4,
        tile_size: TILE_SIZE / 2.0,
        tile_columns: 4,
        spawn_sprite_event: SpawnSpriteEvent {
            sprite_path: animal.source.to_string(),
            size: animal.size,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.0),
                ..Default::default()
            },
        },
    });
}
