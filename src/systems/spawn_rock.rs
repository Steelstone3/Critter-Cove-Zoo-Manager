use crate::{
    assets::images::world::rocks::WorldRock,
    components::rock::Rock,
    events::spawn_sprite_event::SpawnSpriteEvent,
    queries::{
        camera_queries::CameraTransformOrthographicProjectionQuery, window_queries::WindowQuery,
    },
    resources::selected_item::SelectedMenuItem,
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

pub fn spawn_rock(
    mut commands: Commands,
    selected_item: ResMut<SelectedMenuItem>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    windows_queries: Query<WindowQuery>,
    camera_queries: Query<CameraTransformOrthographicProjectionQuery>,
) {
    if selected_item.rock_selection == WorldRock::None {
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

    let rock = Rock::new(selected_item.rock_selection);

    let mut transform = Transform::default();
    transform.translation.z = 1.0;

    if let Some(position) = window_query.window.cursor_position() {
        transform.translation.x = ((position.x - window_query.window.resolution.width() / 2.0)
            * camera_query.projection.scale)
            + camera_query.transform.translation.x;
        transform.translation.y = -((position.y - window_query.window.resolution.height() / 2.0)
            * camera_query.projection.scale)
            + camera_query.transform.translation.y;
    } else {
        return;
    }

    tracing::info!("rock at {:?}", transform.translation);

    spawn_sprite_event.send(SpawnSpriteEvent {
        sprite_path: rock.sprite_path.to_string(),
        size: rock.size,
        transform,
        entity: commands.spawn(rock).id(),
    });
}
