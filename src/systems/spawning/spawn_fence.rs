use crate::{
    assets::images::world::fence_sprites::FenceSprite,
    components::fence::Fence,
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
    },
    input::{mouse::MouseButton, ButtonInput},
    transform::components::Transform,
    utils::tracing,
};

pub fn spawn_fence(
    mut commands: Commands,
    selected_item: ResMut<SelectedMenuItem>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    windows_queries: Query<WindowQuery>,
    camera_queries: Query<CameraTransformOrthographicProjectionQuery>,
) {
    if selected_item.fence_selection == FenceSprite::None {
        return;
    }

    let Ok(window_query) = windows_queries.get_single() else {
        return;
    };

    let Ok(camera_query) = camera_queries.get_single() else {
        return;
    };

    if !mouse_button_input.clear_just_pressed(MouseButton::Right) {
        return;
    }

    let fence = Fence::new(selected_item.fence_selection);

    let mut transform = Transform::default();
    transform.translation.z = fence.z_index;

    if let Some(position) = window_query.window.cursor_position() {
        get_tile_location(&mut transform, position, window_query, camera_query);
    } else {
        return;
    }

    tracing::info!("fence at {:?}", transform.translation);

    spawn_sprite_event.send(SpawnSpriteEvent {
        sprite_path: fence.sprite_path.to_string(),
        size: fence.size,
        transform,
        entity: commands.spawn(fence).id(),
    });
}
