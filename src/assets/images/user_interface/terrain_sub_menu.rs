use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum TerrainSubMenu {
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

impl Display for TerrainSubMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TerrainSubMenu::DarkGrass1 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_1_icon.png"
            ),
            TerrainSubMenu::DarkGrass2 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_2_icon.png"
            ),
            TerrainSubMenu::DarkGrass3 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_3_icon.png"
            ),
            TerrainSubMenu::DarkGrass4 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_4_icon.png"
            ),
            TerrainSubMenu::DarkGrass5 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_5_icon.png"
            ),
            TerrainSubMenu::DarkGrass6 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_6_icon.png"
            ),
            TerrainSubMenu::DarkGrass7 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_7_icon.png"
            ),
            TerrainSubMenu::DarkGrass8 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_8_icon.png"
            ),
            TerrainSubMenu::DarkGrass9 => write!(
                f,
                "images/user_interface/terrain_menu/dark_grass_9_icon.png"
            ),
            TerrainSubMenu::Grass1 => {
                write!(f, "images/user_interface/terrain_menu/grass_1_icon.png")
            }
            TerrainSubMenu::Grass2 => {
                write!(f, "images/user_interface/terrain_menu/grass_2_icon.png")
            }
            TerrainSubMenu::Grass3 => {
                write!(f, "images/user_interface/terrain_menu/grass_3_icon.png")
            }
            TerrainSubMenu::Grass4 => {
                write!(f, "images/user_interface/terrain_menu/grass_4_icon.png")
            }
            TerrainSubMenu::Grass5 => {
                write!(f, "images/user_interface/terrain_menu/grass_5_icon.png")
            }
            TerrainSubMenu::Grass6 => {
                write!(f, "images/user_interface/terrain_menu/grass_6_icon.png")
            }
            TerrainSubMenu::LightGrass1 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_1_icon.png"
            ),
            TerrainSubMenu::LightGrass2 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_2_icon.png"
            ),
            TerrainSubMenu::LightGrass3 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_3_icon.png"
            ),
            TerrainSubMenu::LightGrass4 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_4_icon.png"
            ),
            TerrainSubMenu::LightGrass5 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_5_icon.png"
            ),
            TerrainSubMenu::LightGrass6 => write!(
                f,
                "images/user_interface/terrain_menu/light_grass_6_icon.png"
            ),
            TerrainSubMenu::Savanah1 => {
                write!(f, "images/user_interface/terrain_menu/savanah_1_icon.png")
            }
            TerrainSubMenu::Savanah2 => {
                write!(f, "images/user_interface/terrain_menu/savanah_2_icon.png")
            }
            TerrainSubMenu::Savanah3 => {
                write!(f, "images/user_interface/terrain_menu/savanah_3_icon.png")
            }
            TerrainSubMenu::Savanah4 => {
                write!(f, "images/user_interface/terrain_menu/savanah_4_icon.png")
            }
            TerrainSubMenu::VeryLightGrass1 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_1_icon.png"
            ),
            TerrainSubMenu::VeryLightGrass2 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_2_icon.png"
            ),
            TerrainSubMenu::VeryLightGrass3 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_3_icon.png"
            ),
            TerrainSubMenu::VeryLightGrass4 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_4_icon.png"
            ),
            TerrainSubMenu::VeryLightGrass5 => write!(
                f,
                "images/user_interface/terrain_menu/very_light_grass_5_icon.png"
            ),
            TerrainSubMenu::Water1 => {
                write!(f, "images/user_interface/terrain_menu/water_1_icon.png")
            }
            TerrainSubMenu::Water2 => {
                write!(f, "images/user_interface/terrain_menu/water_2_icon.png")
            }
            TerrainSubMenu::Water3 => {
                write!(f, "images/user_interface/terrain_menu/water_3_icon.png")
            }
            TerrainSubMenu::Water4 => {
                write!(f, "images/user_interface/terrain_menu/water_4_icon.png")
            }
            TerrainSubMenu::Water5 => {
                write!(f, "images/user_interface/terrain_menu/water_5_icon.png")
            }
            TerrainSubMenu::Water6 => {
                write!(f, "images/user_interface/terrain_menu/water_6_icon.png")
            }
            TerrainSubMenu::Water7 => {
                write!(f, "images/user_interface/terrain_menu/water_7_icon.png")
            }
            TerrainSubMenu::Water8 => {
                write!(f, "images/user_interface/terrain_menu/water_8_icon.png")
            }
            TerrainSubMenu::Water9 => {
                write!(f, "images/user_interface/terrain_menu/water_9_icon.png")
            }
            TerrainSubMenu::Water10 => {
                write!(f, "images/user_interface/terrain_menu/water_10_icon.png")
            }
            TerrainSubMenu::Water11 => {
                write!(f, "images/user_interface/terrain_menu/water_11_icon.png")
            }
            TerrainSubMenu::Water12 => {
                write!(f, "images/user_interface/terrain_menu/water_12_icon.png")
            }
            TerrainSubMenu::Water13 => {
                write!(f, "images/user_interface/terrain_menu/water_13_icon.png")
            }
            TerrainSubMenu::Water14 => {
                write!(f, "images/user_interface/terrain_menu/water_14_icon.png")
            }
        }
    }
}
