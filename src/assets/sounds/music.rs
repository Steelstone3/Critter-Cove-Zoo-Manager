use std::fmt::Display;

#[allow(dead_code)]
pub enum Music {
    DessertBiome,
    GrasslandBiome,
    JungleBiome,
    MountainBiome,
    OceanBiome,
}

impl Display for Music {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Music::DessertBiome => {
                write!(f, "sounds/music/dessert_biome.ogg")
            }
            Music::GrasslandBiome => {
                write!(f, "sounds/music/grassland_biome.ogg")
            }
            Music::JungleBiome => {
                write!(f, "sounds/music/jungle_biome.ogg")
            }
            Music::MountainBiome => {
                write!(f, "sounds/music/mountain_biome.ogg")
            }
            Music::OceanBiome => {
                write!(f, "sounds/music/ocean_biome.ogg")
            }
        }
    }
}
