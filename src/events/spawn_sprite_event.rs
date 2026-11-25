use bevy::{
    ecs::{entity::Entity, message::Message},
    math::Vec2,
    transform::components::Transform,
};

#[derive(Message)]
pub struct SpawnSpriteEvent {
    pub sprite_path: String,
    pub size: Vec2,
    pub transform: Transform,
    pub entity: Entity,
}
