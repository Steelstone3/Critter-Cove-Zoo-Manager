use bevy::ecs::event::EventWriter;

use crate::events::user_interface_event::UserInterfaceEvent;

pub fn spawn_user_interface(mut user_interface_event: EventWriter<UserInterfaceEvent>) {
    user_interface_event.send(UserInterfaceEvent {});
}
