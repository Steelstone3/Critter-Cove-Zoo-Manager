use std::fmt::Display;

#[allow(dead_code)]
pub enum WorldTerrain {
    DarkGrass1,
    DarkGrass2,
    DarkGrass3,
    DarkGrass4,
    DarkGrass5,
    DarkGrass6,
    DarkGrass7,
    DarkGrass8,
    Grass1,
    Grass2,
    Grass3,
    Grass4,
    Grass5,
    None,
}

impl Display for WorldTerrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldTerrain::DarkGrass1 => write!(f, "images/world/grasses/dark_grass_1.png"),
            WorldTerrain::DarkGrass2 => write!(f, "images/world/grasses/dark_grass_2.png"),
            WorldTerrain::DarkGrass3 => write!(f, "images/world/grasses/dark_grass_3.png"),
            WorldTerrain::DarkGrass4 => write!(f, "images/world/grasses/dark_grass_4.png"),
            WorldTerrain::DarkGrass5 => write!(f, "images/world/grasses/dark_grass_5.png"),
            WorldTerrain::DarkGrass6 => write!(f, "images/world/grasses/dark_grass_6.png"),
            WorldTerrain::DarkGrass7 => write!(f, "images/world/grasses/dark_grass_7.png"),
            WorldTerrain::DarkGrass8 => write!(f, "images/world/grasses/dark_grass_8.png"),
            WorldTerrain::Grass1 => write!(f, "images/world/grasses/grass_1.png"),
            WorldTerrain::Grass2 => write!(f, "images/world/grasses/grass_2.png"),
            WorldTerrain::Grass3 => write!(f, "images/world/grasses/grass_3.png"),
            WorldTerrain::Grass4 => write!(f, "images/world/grasses/grass_4.png"),
            WorldTerrain::Grass5 => write!(f, "images/world/grasses/grass_5.png"),
            WorldTerrain::None => write!(f, ""), // No asset selected
        }
    }
}
