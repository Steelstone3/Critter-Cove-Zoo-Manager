use std::ops::Range;

use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

pub fn random_value_f32(seed: u64, range: Range<f32>) -> f32 {
    let mut rng = StdRng::seed_from_u64(seed);

    rng.random_range(range.start..range.end)
}

pub fn generate_seed() -> u64 {
    let seed = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let mut rng = StdRng::from_seed(seed);
    rng.next_u64()
}

#[cfg(test)]
mod random_generator_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1234, -20.0..20.0, -17.736874)]
    #[case(4321, -20.0..20.0, 8.739052)]
    #[case(4321, 0.0..100.0, 71.847626)]
    fn generate_a_random_value_f32(
        #[case] seed: u64,
        #[case] range: Range<f32>,
        #[case] expected: f32,
    ) {
        // When
        let actual = random_value_f32(seed, range);

        // Then
        assert_eq!(expected, actual);
    }
}
