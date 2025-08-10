use rand::Rng;
use rand_distr::{Distribution, StandardUniform};
use std::fmt::Display;

pub enum GameMusic {
    Dessert,
    Grassland,
    Jungle,
    Mountain,
    Ocean,
}

impl Display for GameMusic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMusic::Dessert => {
                write!(f, "sounds/music/dessert_biome.ogg")
            }
            GameMusic::Grassland => {
                write!(f, "sounds/music/grassland_biome.ogg")
            }
            GameMusic::Jungle => {
                write!(f, "sounds/music/jungle_biome.ogg")
            }
            GameMusic::Mountain => {
                write!(f, "sounds/music/mountain_biome.ogg")
            }
            GameMusic::Ocean => {
                write!(f, "sounds/music/ocean_biome.ogg")
            }
        }
    }
}

impl Distribution<GameMusic> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GameMusic {
        match rng.random_range(0..5) {
            0 => GameMusic::Dessert,
            1 => GameMusic::Grassland,
            2 => GameMusic::Jungle,
            3 => GameMusic::Mountain,
            _ => GameMusic::Ocean,
        }
    }
}
