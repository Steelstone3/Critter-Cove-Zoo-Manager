use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum TerrainIcon {
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
}

impl Display for TerrainIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerrainIcon::DarkGrass1 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_1_icon.png"
            ),
            TerrainIcon::DarkGrass2 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_2_icon.png"
            ),
            TerrainIcon::DarkGrass3 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_3_icon.png"
            ),
            TerrainIcon::DarkGrass4 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_4_icon.png"
            ),
            TerrainIcon::DarkGrass5 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_5_icon.png"
            ),
            TerrainIcon::DarkGrass6 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_6_icon.png"
            ),
            TerrainIcon::DarkGrass7 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_7_icon.png"
            ),
            TerrainIcon::DarkGrass8 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_8_icon.png"
            ),
            TerrainIcon::DarkGrass9 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_9_icon.png"
            ),
            TerrainIcon::Grass1 => {
                write!(f, "images/user_interface/terrain_menu/grass_1_icon.png")
            }
            TerrainIcon::Grass2 => {
                write!(f, "images/user_interface/terrain_menu/grass_2_icon.png")
            }
            TerrainIcon::Grass3 => {
                write!(f, "images/user_interface/terrain_menu/grass_3_icon.png")
            }
            TerrainIcon::Grass4 => {
                write!(f, "images/user_interface/terrain_menu/grass_4_icon.png")
            }
            TerrainIcon::Grass5 => {
                write!(f, "images/user_interface/terrain_menu/grass_5_icon.png")
            }
            TerrainIcon::Grass6 => {
                write!(f, "images/user_interface/terrain_menu/grass_6_icon.png")
            }
            TerrainIcon::LightGrass1 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_1_icon.png"
            ),
            TerrainIcon::LightGrass2 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_2_icon.png"
            ),
            TerrainIcon::LightGrass3 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_3_icon.png"
            ),
            TerrainIcon::LightGrass4 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_4_icon.png"
            ),
            TerrainIcon::LightGrass5 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_5_icon.png"
            ),
            TerrainIcon::LightGrass6 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_6_icon.png"
            ),
            TerrainIcon::Savanah1 => {
                write!(f, "images/user_interface/terrain_menu/savanah_1_icon.png")
            }
            TerrainIcon::Savanah2 => {
                write!(f, "images/user_interface/terrain_menu/savanah_2_icon.png")
            }
            TerrainIcon::Savanah3 => {
                write!(f, "images/user_interface/terrain_menu/savanah_3_icon.png")
            }
            TerrainIcon::Savanah4 => {
                write!(f, "images/user_interface/terrain_menu/savanah_4_icon.png")
            }
            TerrainIcon::VeryLightGrass1 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_1_icon.png"
            ),
            TerrainIcon::VeryLightGrass2 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_2_icon.png"
            ),
            TerrainIcon::VeryLightGrass3 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_3_icon.png"
            ),
            TerrainIcon::VeryLightGrass4 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_4_icon.png"
            ),
            TerrainIcon::VeryLightGrass5 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_5_icon.png"
            ),
            TerrainIcon::Water1 => {
                write!(f, "images/user_interface/terrain_menu/water_1_icon.png")
            }
            TerrainIcon::Water2 => {
                write!(f, "images/user_interface/terrain_menu/water_2_icon.png")
            }
            TerrainIcon::Water3 => {
                write!(f, "images/user_interface/terrain_menu/water_3_icon.png")
            }
            TerrainIcon::Water4 => {
                write!(f, "images/user_interface/terrain_menu/water_4_icon.png")
            }
            TerrainIcon::Water5 => {
                write!(f, "images/user_interface/terrain_menu/water_5_icon.png")
            }
            TerrainIcon::Water6 => {
                write!(f, "images/user_interface/terrain_menu/water_6_icon.png")
            }
            TerrainIcon::Water7 => {
                write!(f, "images/user_interface/terrain_menu/water_7_icon.png")
            }
            TerrainIcon::Water8 => {
                write!(f, "images/user_interface/terrain_menu/water_8_icon.png")
            }
            TerrainIcon::Water9 => {
                write!(f, "images/user_interface/terrain_menu/water_9_icon.png")
            }
            TerrainIcon::Water10 => {
                write!(f, "images/user_interface/terrain_menu/water_10_icon.png")
            }
            TerrainIcon::Water11 => {
                write!(f, "images/user_interface/terrain_menu/water_11_icon.png")
            }
            TerrainIcon::Water12 => {
                write!(f, "images/user_interface/terrain_menu/water_12_icon.png")
            }
            TerrainIcon::Water13 => {
                write!(f, "images/user_interface/terrain_menu/water_13_icon.png")
            }
            TerrainIcon::Water14 => {
                write!(f, "images/user_interface/terrain_menu/water_14_icon.png")
            }
        }
    }
}
