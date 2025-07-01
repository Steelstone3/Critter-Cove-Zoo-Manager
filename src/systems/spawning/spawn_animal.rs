use crate::{
    assets::images::animal_sprites::AnimalSprite,
    components::animal::Animal,
    events::{
        spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sprite_event::SpawnSpriteEvent,
    },
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
    },
    input::{mouse::MouseButton, ButtonInput},
    log::tracing,
    transform::components::Transform,
};

pub fn spawn_animal(
    mut commands: Commands,
    selected_item: ResMut<SelectedMenuItem>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>,
    windows_query: Query<WindowQuery>,
    camera_queries: Query<CameraTransformOrthographicProjectionQuery>,
) {
    if selected_item.animal_selection == AnimalSprite::None {
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

    let mut animal = Animal::new_16(selected_item.animal_selection);

    if selected_item.animal_selection == AnimalSprite::Gorilla
        || selected_item.animal_selection == AnimalSprite::Moose
        || selected_item.animal_selection == AnimalSprite::RearingNightmare
        || selected_item.animal_selection == AnimalSprite::StormGiant
    {
        animal = Animal::new_32(selected_item.animal_selection)
    }

    let mut transform = Transform::default();
    transform.translation.z = animal.z_index;

    if let Some(position) = window_query.window.cursor_position() {
        get_cursor_location(&mut transform, position, window_query, camera_query);
    } else {
        return;
    }

    tracing::info!("animal at {:?}", transform.translation);

    spawn_animated_sprite_event.write(SpawnAnimatedSpriteEvent {
        frame_timing: animal.frame_timing,
        frame_count: animal.frame_count,
        tile_size: animal.tile_size,
        tile_columns: animal.frame_count as u32,
        spawn_sprite_event: SpawnSpriteEvent {
            sprite_path: animal.sprite_path.to_string(),
            size: animal.size,
            transform,
            entity: commands.spawn(animal).id(),
        },
    });
}
