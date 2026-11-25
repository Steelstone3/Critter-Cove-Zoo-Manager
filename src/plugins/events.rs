use bevy::prelude::{App, Plugin};

use crate::events::{
    despawn_entity_event::DespawnEntityEvent,
    spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sound_event::SpawnSoundEvent,
    spawn_sprite_event::SpawnSpriteEvent,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnAnimatedSpriteEvent>()
            .add_message::<SpawnSpriteEvent>()
            .add_message::<DespawnEntityEvent>()
            .add_message::<SpawnSoundEvent>();
    }
}
