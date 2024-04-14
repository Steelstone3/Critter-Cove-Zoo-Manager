use crate::{
    assets::images::animal::ZooAnimal,
    components::animal::Animal,
    events::{
        spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sprite_event::SpawnSpriteEvent,
    },
    queries::window_queries::WindowQuery,
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

pub fn spawn_animal(
    mut commands: Commands,
    selected_item: ResMut<SelectedMenuItem>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>,
    windows_query: Query<WindowQuery>,
) {
    if selected_item.animal_selection == ZooAnimal::None {
        return;
    }

    let Ok(window_query) = windows_query.get_single() else {
        return;
    };

    if !mouse_button_input.clear_just_pressed(MouseButton::Right) {
        return;
    }

    let mut animal = Animal::new_16(selected_item.animal_selection);

    if selected_item.animal_selection == ZooAnimal::Gorilla
        || selected_item.animal_selection == ZooAnimal::Moose
        || selected_item.animal_selection == ZooAnimal::RearingNightmare
        || selected_item.animal_selection == ZooAnimal::StormGiant
    {
        animal = Animal::new_32(selected_item.animal_selection)
    }

    let mut transform = Transform::default();
    transform.translation.z = 1.0;

    // TODO Zoom and moving the camera effect the sync of this
    if let Some(position) = window_query.window.cursor_position() {
        transform.translation.x = position.x - window_query.window.resolution.width() / 2.0;
        transform.translation.y = -(position.y - window_query.window.resolution.height() / 2.0);
    } else {
        return;
    }

    tracing::info!("animal at {:?}", transform.translation);

    spawn_animated_sprite_event.send(SpawnAnimatedSpriteEvent {
        frame_timing: animal.frame_timing,
        frame_count: animal.frame_count,
        tile_size: animal.tile_size,
        tile_columns: animal.frame_count,
        spawn_sprite_event: SpawnSpriteEvent {
            sprite_path: animal.sprite_path.to_string(),
            size: animal.size,
            transform,
            entity: commands.spawn(animal).id(),
        },
    });
}
