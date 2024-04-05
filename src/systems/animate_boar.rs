use bevy::{
    ecs::system::{Query, Res},
    sprite::TextureAtlas,
    time::Time,
};

use crate::components::animation_timer::AnimationTimer;

pub fn animate_sprites(
    time: Res<Time>,
    mut animations_query: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
) {
    for mut animation_query in animations_query.iter_mut() {
        animation_query.0.timer.tick(time.delta());

        if animation_query.0.timer.just_finished() {
            animation_query.1.index += 1;
            if animation_query.1.index >= animation_query.0.frame_count {
                animation_query.1.index = 0;
            }
        }
    }
}
