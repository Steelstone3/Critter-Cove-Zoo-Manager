use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum WorldTerrain {
    DarkGrass1,
    DarkGrass2,
    DarkGrass3,
    DarkGrass4,
    DarkGrass5,
    DarkGrass6,
    DarkGrass7,
    DarkGrass8,
    DarkGrass9,
    Grass1,
    Grass2,
    Grass3,
    Grass4,
    Grass5,
    Grass6,
    LightGrass1,
    LightGrass2,
    LightGrass3,
    LightGrass4,
    LightGrass5,
    LightGrass6,
    Savanah1,
    Savanah2,
    Savanah3,
    Savanah4,
    VeryLightGrass1,
    VeryLightGrass2,
    VeryLightGrass3,
    VeryLightGrass4,
    VeryLightGrass5,
    Water1,
    Water2,
    Water3,
    Water4,
    Water5,
    Water6,
    Water7,
    Water8,
    Water9,
    Water10,
    Water11,
    Water12,
    Water13,
    Water14,
    None,
}

impl Display for WorldTerrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldTerrain::DarkGrass1 => write!(f, "images/world/terrains/dark_grass_1.png"),
            WorldTerrain::DarkGrass2 => write!(f, "images/world/terrains/dark_grass_2.png"),
            WorldTerrain::DarkGrass3 => write!(f, "images/world/terrains/dark_grass_3.png"),
            WorldTerrain::DarkGrass4 => write!(f, "images/world/terrains/dark_grass_4.png"),
            WorldTerrain::DarkGrass5 => write!(f, "images/world/terrains/dark_grass_5.png"),
            WorldTerrain::DarkGrass6 => write!(f, "images/world/terrains/dark_grass_6.png"),
            WorldTerrain::DarkGrass7 => write!(f, "images/world/terrains/dark_grass_7.png"),
            WorldTerrain::DarkGrass8 => write!(f, "images/world/terrains/dark_grass_8.png"),
            WorldTerrain::DarkGrass9 => write!(f, "images/world/terrains/dark_grass_9.png"),
            WorldTerrain::Grass1 => write!(f, "images/world/terrains/grass_1.png"),
            WorldTerrain::Grass2 => write!(f, "images/world/terrains/grass_2.png"),
            WorldTerrain::Grass3 => write!(f, "images/world/terrains/grass_3.png"),
            WorldTerrain::Grass4 => write!(f, "images/world/terrains/grass_4.png"),
            WorldTerrain::Grass5 => write!(f, "images/world/terrains/grass_5.png"),
            WorldTerrain::Grass6 => write!(f, "images/world/terrains/grass_6.png"),
            WorldTerrain::LightGrass1 => write!(f, "images/world/terrains/light_grass_1.png"),
            WorldTerrain::LightGrass2 => write!(f, "images/world/terrains/light_grass_2.png"),
            WorldTerrain::LightGrass3 => write!(f, "images/world/terrains/light_grass_3.png"),
            WorldTerrain::LightGrass4 => write!(f, "images/world/terrains/light_grass_4.png"),
            WorldTerrain::LightGrass5 => write!(f, "images/world/terrains/light_grass_5.png"),
            WorldTerrain::LightGrass6 => write!(f, "images/world/terrains/light_grass_6.png"),
            WorldTerrain::Savanah1 => write!(f, "images/world/terrains/savanah_1.png"),
            WorldTerrain::Savanah2 => write!(f, "images/world/terrains/savanah_2.png"),
            WorldTerrain::Savanah3 => write!(f, "images/world/terrains/savanah_3.png"),
            WorldTerrain::Savanah4 => write!(f, "images/world/terrains/savanah_4.png"),
            WorldTerrain::VeryLightGrass1 => {
                write!(f, "images/world/terrains/very_light_grass_1.png")
            }
            WorldTerrain::VeryLightGrass2 => {
                write!(f, "images/world/terrains/very_light_grass_2.png")
            }
            WorldTerrain::VeryLightGrass3 => {
                write!(f, "images/world/terrains/very_light_grass_3.png")
            }
            WorldTerrain::VeryLightGrass4 => {
                write!(f, "images/world/terrains/very_light_grass_4.png")
            }
            WorldTerrain::VeryLightGrass5 => {
                write!(f, "images/world/terrains/very_light_grass_5.png")
            }
            WorldTerrain::Water1 => write!(f, "images/world/terrains/water_1.png"),
            WorldTerrain::Water2 => write!(f, "images/world/terrains/water_2.png"),
            WorldTerrain::Water3 => write!(f, "images/world/terrains/water_3.png"),
            WorldTerrain::Water4 => write!(f, "images/world/terrains/water_4.png"),
            WorldTerrain::Water5 => write!(f, "images/world/terrains/water_5.png"),
            WorldTerrain::Water6 => write!(f, "images/world/terrains/water_6.png"),
            WorldTerrain::Water7 => write!(f, "images/world/terrains/water_7.png"),
            WorldTerrain::Water8 => write!(f, "images/world/terrains/water_8.png"),
            WorldTerrain::Water9 => write!(f, "images/world/terrains/water_9.png"),
            WorldTerrain::Water10 => write!(f, "images/world/terrains/water_10.png"),
            WorldTerrain::Water11 => write!(f, "images/world/terrains/water_11.png"),
            WorldTerrain::Water12 => write!(f, "images/world/terrains/water_12.png"),
            WorldTerrain::Water13 => write!(f, "images/world/terrains/water_13.png"),
            WorldTerrain::Water14 => write!(f, "images/world/terrains/water_14.png"),
            WorldTerrain::None => write!(f, ""), // No asset selected
        }
    }
}
