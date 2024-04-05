use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

use crate::events::event_handlers::{
    despawn_entity::despawn_entity,
    spawn_animated_sprite::spawn_animated_sprite,
    spawn_sound::spawn_sound,
    spawn_sprite::spawn_sprite,
};

pub struct EventHandlersPlugin;

impl Plugin for EventHandlersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_animated_sprite)
            .add_systems(Update, spawn_sprite)
            .add_systems(Update, despawn_entity)
            .add_systems(Update, spawn_sound);
    }
}
