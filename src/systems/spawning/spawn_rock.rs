use crate::{
    assets::images::world::rock_sprites::RockSprite,
    components::rock::Rock,
    events::spawn_sprite_event::SpawnSpriteEvent,
    queries::{
        camera_queries::CameraTransformOrthographicProjectionQuery, window_queries::WindowQuery,
    },
    resources::selected_item::SelectedMenuItem,
    systems::controllers::get_location::get_cursor_location,
};
use bevy::{
    ecs::{
        event::EventWriter,
        system::{Commands, Query, ResMut},
    }, input::{mouse::MouseButton, ButtonInput}, log::tracing, transform::components::Transform

};

pub fn spawn_rock(
    mut commands: Commands,
    selected_item: ResMut<SelectedMenuItem>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    windows_queries: Query<WindowQuery>,
    camera_queries: Query<CameraTransformOrthographicProjectionQuery>,
) {
    if selected_item.rock_selection == RockSprite::None {
        return;
    }

    let Ok(window_query) = windows_queries.single() else {
        return;
    };

    let Ok(camera_query) = camera_queries.single() else {
        return;
    };

    if !mouse_button_input.clear_just_pressed(MouseButton::Right) {
        return;
    }

    let rock = Rock::new(selected_item.rock_selection);

    let mut transform = Transform::default();
    transform.translation.z = rock.z_index;

    if let Some(position) = window_query.window.cursor_position() {
        get_cursor_location(&mut transform, position, window_query, camera_query);
    } else {
        return;
    }

    tracing::info!("rock at {:?}", transform.translation);

    spawn_sprite_event.write(SpawnSpriteEvent {
        sprite_path: rock.sprite_path.to_string(),
        size: rock.size,
        transform,
        entity: commands.spawn(rock).id(),
    });
}
