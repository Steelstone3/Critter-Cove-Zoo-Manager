use bevy::{
    ecs::{event::EventWriter, system::Res},
    math::Vec3,
    transform::components::Transform,
};

use crate::{
    assets::images::animal::ZooAnimal,
    components::{animal::Animal, constants::TILE_SIZE},
    events::{
        spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sprite_event::SpawnSpriteEvent,
    },
    resources::selected_item::SelectedItem,
};

pub fn spawn_animal(
    selected_item: Res<SelectedItem>,
    mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>,
) {
    let mut animal = Animal::new(selected_item.animal);

    if selected_item.animal == ZooAnimal::Gorilla
        || selected_item.animal == ZooAnimal::Moose
        || selected_item.animal == ZooAnimal::RearingNightmare
        || selected_item.animal == ZooAnimal::StormGiant
    {
        animal = Animal::new_animated(selected_item.animal, TILE_SIZE, 0.1, 8)
    }

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
