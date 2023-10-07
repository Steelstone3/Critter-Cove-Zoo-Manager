use bevy::ecs::query::QueryData;

use crate::components::music::Music;

#[derive(QueryData)]
pub struct MusicQuery {
    pub music: &'static Music,
}
