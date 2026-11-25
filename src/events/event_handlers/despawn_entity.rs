use bevy::ecs::{message::MessageReader, system::Commands};

use crate::events::despawn_entity_event::DespawnEntityEvent;

pub fn despawn_entity(
    mut commands: Commands,
    mut despawn_entity_events: MessageReader<DespawnEntityEvent>,
) {
    for despawn_entity_event in despawn_entity_events.read() {
        commands.entity(despawn_entity_event.entity).despawn()
    }
}
