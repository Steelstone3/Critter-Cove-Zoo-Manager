use std::fmt::Display;

#[allow(dead_code)]
pub enum Music {
    Dessert,
    Grassland,
    Jungle,
    Mountain,
    Ocean,
}

impl Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Music::Dessert => {
                write!(f, "sounds/music/dessert_biome.ogg")
            }
            Music::Grassland => {
                write!(f, "sounds/music/grassland_biome.ogg")
            }
            Music::Jungle => {
                write!(f, "sounds/music/jungle_biome.ogg")
            }
            Music::Mountain => {
                write!(f, "sounds/music/mountain_biome.ogg")
            }
            Music::Ocean => {
                write!(f, "sounds/music/ocean_biome.ogg")
            }
        }
    }
}
