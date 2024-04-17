use crate::{
    assets::images::world::{tree::WorldTree},
    components::{tree::Tree},
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

pub fn spawn_tree(
    mut commands: Commands,
    selected_item: ResMut<SelectedMenuItem>,
    mut mouse_button_input: ResMut<ButtonInput<MouseButton>>,
    mut spawn_sprite_event: EventWriter<SpawnSpriteEvent>,
    windows_queries: Query<WindowQuery>,
    camera_queries: Query<CameraTransformOrthographicProjectionQuery>,
) {
    if selected_item.tree_selection == WorldTree::None {
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

    let tree = Tree::new(selected_item.tree_selection);

    let mut transform = Transform::default();
    transform.translation.z = 1.0;

    // TODO Extract this "spawn at mouse pointer" system (used in Animals, Trees and Rocks)
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

    tracing::info!("tree at {:?}", transform.translation);

    spawn_sprite_event.send(SpawnSpriteEvent {
        sprite_path: tree.sprite_path.to_string(),
        size: tree.size,
        transform,
        entity: commands.spawn(tree).id(),
    });
}
