use std::fmt::Display;

pub enum WorldTerrain {
    Grass,
}

impl Display for WorldTerrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldTerrain::Grass => write!(f, "images/world/terrain.png"),
        }
    }
}
