use bevy::{ecs::event::EventWriter, math::Vec3, transform::components::Transform};

use crate::{components::animal::Animal, events::spawn_sprite_event::SpawnSpriteEvent};


pub fn spawn_terrain(mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>) {
    let animal = Animal::new_boar();

    spawn_sprite_event.send(SpawnSpriteEvent {
        sprite_path: animal.sprite_path.to_string(),
        size: animal.size,
        transform: Transform {
            translation: Vec3::new(32.0,32.0,0.0),
            ..Default::default()
        },
    });
}
