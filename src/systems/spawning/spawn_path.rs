use crate::{
    assets::images::world::path_sprites::PathSprite,
    components::path::Path,
    events::spawn_sprite_event::SpawnSpriteEvent,
    queries::{
        camera_queries::CameraTransformOrthographicProjectionQuery, window_queries::WindowQuery,
    },
    resources::selected_item::SelectedMenuItem,
    systems::controllers::get_location::get_tile_location,
};
use bevy::{
    ecs::{
        event::EventWriter,
        system::{Commands, Query, ResMut},
    }, input::{mouse::MouseButton, ButtonInput}, log::tracing, transform::components::Transform
};

pub fn spawn_path(
    mut commands: Commands,
    selected_item: ResMut<SelectedMenuItem>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    windows_query: Query<WindowQuery>,
    camera_queries: Query<CameraTransformOrthographicProjectionQuery>,
) {
    if selected_item.path_selection == PathSprite::None {
        return;
    }

    let Ok(window_query) = windows_query.single() else {
        return;
    };

    let Ok(camera_query) = camera_queries.single() else {
        return;
    };

    if !mouse_button_input.clear_just_pressed(MouseButton::Right) {
        return;
    }

    let path = Path::new(selected_item.path_selection);

    let mut transform = Transform::default();
    transform.translation.z = path.z_index;

    if let Some(position) = window_query.window.cursor_position() {
        get_tile_location(&mut transform, position, window_query, camera_query);
    } else {
        return;
    }

    tracing::info!("path at {:?}", transform.translation);

    spawn_sprite_event.write(SpawnSpriteEvent {
        sprite_path: path.sprite_path.to_string(),
        size: path.size,
        transform,
        entity: commands.spawn(path).id(),
    });
}
