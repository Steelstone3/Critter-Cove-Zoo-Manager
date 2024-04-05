use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct TileSize {
    pub size: f32,
}
