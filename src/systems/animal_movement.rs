use super::controllers::random_generator::{generate_seed, random_value_f32};
use crate::{
    components::constants::{MAP_SIZE, TILE_SIZE},
    queries::animal_queries::MutableAnimalTransformQuery,
};
use bevy::{
    ecs::system::{Query, Res},
    math::Vec3,
    time::Time,
};

pub fn animal_movement(mut animal_queries: Query<MutableAnimalTransformQuery>, time: Res<Time>) {
    animal_queries.par_iter_mut().for_each(|mut animal_query| {
        let speed = animal_query.animal.speed * time.delta_secs();

        // Check if the animal has reached its destination
        let distance_to_destination =
            (animal_query.transform.translation - animal_query.animal.destination).length();
        let is_at_destination = distance_to_destination
            < animal_query.animal.destination.x + TILE_SIZE
            || distance_to_destination < animal_query.animal.destination.y + TILE_SIZE;

        if is_at_destination {
            // If the animal has reached its destination, generate a new random destination
            let new_destination = Vec3::new(
                random_value_f32(generate_seed(), -MAP_SIZE..MAP_SIZE),
                random_value_f32(generate_seed(), -MAP_SIZE..MAP_SIZE),
                1.0,
            );

            animal_query.animal.destination = new_destination;
        }

        // Calculate the direction towards the destination
        let direction =
            (animal_query.animal.destination - animal_query.transform.translation).normalize();
        let translation_delta = direction * speed;

        animal_query.transform.translation += translation_delta;
    });
}
