use crate::{
    assets::images::world::tree_sprites::TreeSprite,
    components::tree::Tree,
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
    if selected_item.tree_selection == TreeSprite::None {
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

    let mut tree = Tree::new_128(selected_item.tree_selection);

    if selected_item.tree_selection == TreeSprite::Bush1
        || selected_item.tree_selection == TreeSprite::Bush2
        || selected_item.tree_selection == TreeSprite::TallGrass1
        || selected_item.tree_selection == TreeSprite::TallGrass2
        || selected_item.tree_selection == TreeSprite::TallGrass3
    {
        tree = Tree::new_32(selected_item.tree_selection);
    }

    let mut transform = Transform::default();
    transform.translation.z = tree.z_index;

    if let Some(position) = window_query.window.cursor_position() {
        get_cursor_location(&mut transform, position, window_query, camera_query);
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
