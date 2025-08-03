use crate::{
    components::animation_timer::AnimationTimer,
    events::spawn_animated_sprite_event::SpawnAnimatedSpriteEvent,
};
use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        event::EventReader,
        system::{Commands, Res, ResMut},
    },
    image::{TextureAtlas, TextureAtlasLayout},
    math::UVec2,
    sprite::Sprite,
};

pub fn spawn_animated_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawn_animated_sprite_events: EventReader<SpawnAnimatedSpriteEvent>,
) {
    for spawn_animated_sprite_event in spawn_animated_sprite_events.read() {
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(
                spawn_animated_sprite_event.tile_size,
                spawn_animated_sprite_event.tile_size,
            ),
            spawn_animated_sprite_event.tile_columns,
            1,
            None,
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        if let Ok(mut entity) =
            commands.get_entity(spawn_animated_sprite_event.spawn_sprite_event.entity)
        {
            // create sprite
            let mut sprite = Sprite::from_atlas_image(
                asset_server.load(
                    spawn_animated_sprite_event
                        .spawn_sprite_event
                        .sprite_path
                        .to_string(),
                ),
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            );
            sprite.custom_size = Some(spawn_animated_sprite_event.spawn_sprite_event.size);

            // create animation timer
            let animation_timer = AnimationTimer::new(
                spawn_animated_sprite_event.frame_timing,
                spawn_animated_sprite_event.frame_count,
            );

            // TODO AH find a way of doing the animation timer
            entity.insert((
                sprite,
                spawn_animated_sprite_event.spawn_sprite_event.transform,
                animation_timer,
            ));

            //     (
            //     SpriteBundle {
            //         sprite: Sprite {
            //             custom_size: Some(spawn_animated_sprite_event.spawn_sprite_event.size),
            //             ..Default::default()
            //         },
            //         texture: asset_server.load(
            //             spawn_animated_sprite_event
            //                 .spawn_sprite_event
            //                 .sprite_path
            //                 .to_string(),
            //         ),
            //         transform: spawn_animated_sprite_event.spawn_sprite_event.transform,
            //         ..Default::default()
            //     },
            //     AnimationTimer::new(
            //         spawn_animated_sprite_event.frame_timing,
            //         spawn_animated_sprite_event.frame_count,
            //     ),
            //     TextureAtlas {
            //         layout: texture_atlas_layout,
            //         index: 0,
            //     },
            // )
        }
    }
}
