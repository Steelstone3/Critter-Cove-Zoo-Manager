use bevy::ecs::system::Query;

use crate::queries::animal_queries::MutableAnimalTransformQuery;

#[allow(dead_code)]
pub fn animal_movement(mut animal_queries: Query<MutableAnimalTransformQuery>) {
    animal_queries.par_iter_mut().for_each(|_animal_query| {

        // let ship_speed = ai_ship.velocity * time.delta_seconds();
        // let movement_direction = transform.rotation * Vec3::Y;
        // let translation_delta = movement_direction * ship_speed;

        // let next_translation = transform.translation + translation_delta;

        // if next_translation.y > space_zone_border.top_border
        //     || next_translation.x < space_zone_border.left_border
        //     || next_translation.y < space_zone_border.bottom_border
        //     || next_translation.x > space_zone_border.right_border
        // {
        //     transform.rotate(Quat::from_axis_angle(
        //         Vec3::new(0.0, 0.0, 1.0),
        //         random_value_f32(generate_seed(), 150.0..210.0),
        //     ))
        // } else {
        //     transform.translation += translation_delta;
        // }
    });
}
