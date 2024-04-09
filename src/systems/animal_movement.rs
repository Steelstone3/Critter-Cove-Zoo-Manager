use super::controllers::random_generator::{generate_seed, random_value_f32};
use crate::{
    components::constants::MAP_SIZE, queries::animal_queries::MutableAnimalTransformQuery,
};
use bevy::{
    ecs::system::{Query, Res},
    math::{Quat, Vec3},
    time::Time,
};

pub fn animal_movement(mut animal_queries: Query<MutableAnimalTransformQuery>, time: Res<Time>) {
    animal_queries.par_iter_mut().for_each(|mut animal_query| {
        let speed = animal_query.animal.speed * time.delta_seconds();
        let direction = animal_query.transform.rotation * Vec3::Y;
        let translation_delta = direction * speed;

        let next_translation_delta = animal_query.transform.translation + translation_delta;

        let is_top_world_border = next_translation_delta.y > MAP_SIZE;
        let is_right_world_border = next_translation_delta.x > MAP_SIZE;
        let is_bottom_world_border = next_translation_delta.y < -MAP_SIZE;
        let is_left_world_border = next_translation_delta.x < -MAP_SIZE;

        if is_top_world_border
            || is_right_world_border
            || is_bottom_world_border
            || is_left_world_border
        {
            animal_query.transform.rotate(Quat::from_axis_angle(
                Vec3::new(0.0, 0.0, 1.0),
                random_value_f32(generate_seed(), 150.0..210.0),
            ))
        } else {
            animal_query.transform.translation += translation_delta;
        }
    });
}

// let ship_speed = starship.starship.velocity * time.delta_seconds();
// let movement_direction = starship.transform.rotation * Vec3::Y;
// let translation_delta = movement_direction * ship_speed;

// let next_translation = starship.transform.translation + translation_delta;

// if next_translation.y > space_zone_border.top_border
//     || next_translation.x < space_zone_border.left_border
//     || next_translation.y < space_zone_border.bottom_border
//     || next_translation.x > space_zone_border.right_border
// {
//     starship.transform.rotate(Quat::from_axis_angle(
//         Vec3::new(0.0, 0.0, 1.0),
//         random_value_f32(generate_seed(), 150.0..210.0),
//     ))
// } else {
//     starship.transform.translation += translation_delta;
// }
