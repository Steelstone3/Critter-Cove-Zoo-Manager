use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

use crate::events::event_handlers::{
    despawn_entity_event_handler::despawn_entity_event_handler,
    spawn_animated_sprite_event_handler::spawn_animated_sprite_event_handler,
    spawn_sound_event_handler::spawn_sound_event_handler,
    spawn_sprite_event_handler::spawn_sprite_event_handler,
};

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_animated_sprite_event_handler)
            .add_systems(Update, spawn_sprite_event_handler)
            .add_systems(Update, despawn_entity_event_handler)
            .add_systems(Update, spawn_sound_event_handler);
    }
}
