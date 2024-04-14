use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
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
