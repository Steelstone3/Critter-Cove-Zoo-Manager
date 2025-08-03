use bevy::{
    ecs::{event::EventWriter, system::Commands},
    math::Vec3,
    transform::components::Transform,
};

use crate::{
    assets::images::world::terrain_sprites::TerrainSprite,
    components::{
        constants::{MAP_TILES, TILE_SIZE},
        terrain::Terrain,
    },
    events::spawn_sprite_event::SpawnSpriteEvent,
};

pub fn spawn_world_terrain(
    mut commands: Commands,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
) {
    let terrain = Terrain::new(TerrainSprite::Grass1);

    for x in MAP_TILES {
        for y in MAP_TILES {
            spawn_sprite_event.write(SpawnSpriteEvent {
                sprite_path: terrain.sprite_path.to_string(),
                size: terrain.size,
                transform: Transform {
                    translation: Vec3::new(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        terrain.z_index,
                    ),
                    ..Default::default()
                },
                entity: commands.spawn(terrain).id(),
            });
        }
    }
}
