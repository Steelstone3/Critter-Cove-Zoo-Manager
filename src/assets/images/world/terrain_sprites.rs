use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum TerrainSprite {
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

impl Display for TerrainSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerrainSprite::DarkGrass1 => write!(f, "images/world/terrains/dark_grass_1.png"),
            TerrainSprite::DarkGrass2 => write!(f, "images/world/terrains/dark_grass_2.png"),
            TerrainSprite::DarkGrass3 => write!(f, "images/world/terrains/dark_grass_3.png"),
            TerrainSprite::DarkGrass4 => write!(f, "images/world/terrains/dark_grass_4.png"),
            TerrainSprite::DarkGrass5 => write!(f, "images/world/terrains/dark_grass_5.png"),
            TerrainSprite::DarkGrass6 => write!(f, "images/world/terrains/dark_grass_6.png"),
            TerrainSprite::DarkGrass7 => write!(f, "images/world/terrains/dark_grass_7.png"),
            TerrainSprite::DarkGrass8 => write!(f, "images/world/terrains/dark_grass_8.png"),
            TerrainSprite::DarkGrass9 => write!(f, "images/world/terrains/dark_grass_9.png"),
            TerrainSprite::Grass1 => write!(f, "images/world/terrains/grass_1.png"),
            TerrainSprite::Grass2 => write!(f, "images/world/terrains/grass_2.png"),
            TerrainSprite::Grass3 => write!(f, "images/world/terrains/grass_3.png"),
            TerrainSprite::Grass4 => write!(f, "images/world/terrains/grass_4.png"),
            TerrainSprite::Grass5 => write!(f, "images/world/terrains/grass_5.png"),
            TerrainSprite::Grass6 => write!(f, "images/world/terrains/grass_6.png"),
            TerrainSprite::LightGrass1 => write!(f, "images/world/terrains/light_grass_1.png"),
            TerrainSprite::LightGrass2 => write!(f, "images/world/terrains/light_grass_2.png"),
            TerrainSprite::LightGrass3 => write!(f, "images/world/terrains/light_grass_3.png"),
            TerrainSprite::LightGrass4 => write!(f, "images/world/terrains/light_grass_4.png"),
            TerrainSprite::LightGrass5 => write!(f, "images/world/terrains/light_grass_5.png"),
            TerrainSprite::LightGrass6 => write!(f, "images/world/terrains/light_grass_6.png"),
            TerrainSprite::Savanah1 => write!(f, "images/world/terrains/savanah_1.png"),
            TerrainSprite::Savanah2 => write!(f, "images/world/terrains/savanah_2.png"),
            TerrainSprite::Savanah3 => write!(f, "images/world/terrains/savanah_3.png"),
            TerrainSprite::Savanah4 => write!(f, "images/world/terrains/savanah_4.png"),
            TerrainSprite::VeryLightGrass1 => {
                write!(f, "images/world/terrains/very_light_grass_1.png")
            }
            TerrainSprite::VeryLightGrass2 => {
                write!(f, "images/world/terrains/very_light_grass_2.png")
            }
            TerrainSprite::VeryLightGrass3 => {
                write!(f, "images/world/terrains/very_light_grass_3.png")
            }
            TerrainSprite::VeryLightGrass4 => {
                write!(f, "images/world/terrains/very_light_grass_4.png")
            }
            TerrainSprite::VeryLightGrass5 => {
                write!(f, "images/world/terrains/very_light_grass_5.png")
            }
            TerrainSprite::Water1 => write!(f, "images/world/terrains/water_1.png"),
            TerrainSprite::Water2 => write!(f, "images/world/terrains/water_2.png"),
            TerrainSprite::Water3 => write!(f, "images/world/terrains/water_3.png"),
            TerrainSprite::Water4 => write!(f, "images/world/terrains/water_4.png"),
            TerrainSprite::Water5 => write!(f, "images/world/terrains/water_5.png"),
            TerrainSprite::Water6 => write!(f, "images/world/terrains/water_6.png"),
            TerrainSprite::Water7 => write!(f, "images/world/terrains/water_7.png"),
            TerrainSprite::Water8 => write!(f, "images/world/terrains/water_8.png"),
            TerrainSprite::Water9 => write!(f, "images/world/terrains/water_9.png"),
            TerrainSprite::Water10 => write!(f, "images/world/terrains/water_10.png"),
            TerrainSprite::Water11 => write!(f, "images/world/terrains/water_11.png"),
            TerrainSprite::Water12 => write!(f, "images/world/terrains/water_12.png"),
            TerrainSprite::Water13 => write!(f, "images/world/terrains/water_13.png"),
            TerrainSprite::Water14 => write!(f, "images/world/terrains/water_14.png"),
            TerrainSprite::None => write!(f, ""), // No asset selected
        }
    }
}
