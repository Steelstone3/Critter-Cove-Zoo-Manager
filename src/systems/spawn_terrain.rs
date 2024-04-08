use bevy::{ecs::event::EventWriter, math::Vec3, transform::components::Transform};

use crate::{
    components::{
        constants::{MAP_TILES, TILE_SIZE},
        terrain::Terrain,
    },
    events::spawn_sprite_event::SpawnSpriteEvent,
};

pub fn spawn_terrain(mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>) {
    let terrain = Terrain::new_grass();

    for x in MAP_TILES {
        for y in MAP_TILES {
            spawn_sprite_event.send(SpawnSpriteEvent {
                sprite_path: terrain.sprite_path.to_string(),
                size: terrain.size,
                transform: Transform {
                    translation: Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                    ..Default::default()
                },
            });
        }
    }
}
