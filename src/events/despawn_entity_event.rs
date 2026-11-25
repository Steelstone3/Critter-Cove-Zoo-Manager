use bevy::ecs::{entity::Entity, message::Message};

#[derive(Message)]
pub struct DespawnEntityEvent {
    pub entity: Entity,
}
