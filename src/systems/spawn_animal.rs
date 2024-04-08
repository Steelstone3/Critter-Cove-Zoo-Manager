use crate::{
    assets::images::animal::ZooAnimal,
    components::animal::Animal,
    events::{
        spawn_animated_sprite_event::SpawnAnimatedSpriteEvent, spawn_sprite_event::SpawnSpriteEvent,
    },
    resources::selected_item::SelectedMenuItem,
};
use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::ResMut,
    },
    input::{
        mouse::{MouseButton, MouseButtonInput},
        ButtonState,
    },
    math::Vec3,
    transform::components::Transform,
};

pub fn spawn_animal(
    mut selected_item: ResMut<SelectedMenuItem>,
    // input: Res<ButtonInput<KeyCode>>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut spawn_animated_sprite_event: EventWriter<SpawnAnimatedSpriteEvent>,
) {
    for mouse_button_event in mouse_button_events.read() {
        if mouse_button_event.button != MouseButton::Left {
            return;
        }
        if mouse_button_event.state != ButtonState::Pressed {
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

        spawn_animated_sprite_event.send(SpawnAnimatedSpriteEvent {
            frame_timing: animal.frame_timing,
            frame_count: animal.frame_count,
            tile_size: animal.tile_size,
            tile_columns: animal.frame_count,
            spawn_sprite_event: SpawnSpriteEvent {
                sprite_path: animal.sprite_path.to_string(),
                size: animal.size,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
            },
        });

        selected_item.animal_selection = ZooAnimal::None;
    }
}
