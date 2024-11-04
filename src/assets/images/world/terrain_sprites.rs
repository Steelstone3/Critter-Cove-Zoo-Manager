use std::fmt::Display;

use crate::assets::images::user_interface::terrain_icons::TerrainIcon;

#[derive(Debug, PartialEq, Clone, Copy)]
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


impl TerrainSprite {
    pub fn convert_from(icon: TerrainIcon) -> Self {
        match icon {
            TerrainIcon::DarkGrass1 => TerrainSprite::DarkGrass1,
            TerrainIcon::DarkGrass2 => TerrainSprite::DarkGrass2,
            TerrainIcon::DarkGrass3 => TerrainSprite::DarkGrass3,
            TerrainIcon::DarkGrass4 => TerrainSprite::DarkGrass4,
            TerrainIcon::DarkGrass5 => TerrainSprite::DarkGrass5,
            TerrainIcon::DarkGrass6 => TerrainSprite::DarkGrass6,
            TerrainIcon::DarkGrass7 => TerrainSprite::DarkGrass7,
            TerrainIcon::DarkGrass8 => TerrainSprite::DarkGrass8,
            TerrainIcon::DarkGrass9 => TerrainSprite::DarkGrass9,
            TerrainIcon::Grass1 => TerrainSprite::Grass1,
            TerrainIcon::Grass2 => TerrainSprite::Grass2,
            TerrainIcon::Grass3 => TerrainSprite::Grass3,
            TerrainIcon::Grass4 => TerrainSprite::Grass4,
            TerrainIcon::Grass5 => TerrainSprite::Grass5,
            TerrainIcon::Grass6 => TerrainSprite::Grass6,
            TerrainIcon::LightGrass1 => TerrainSprite::LightGrass1,
            TerrainIcon::LightGrass2 => TerrainSprite::LightGrass2,
            TerrainIcon::LightGrass3 => TerrainSprite::LightGrass3,
            TerrainIcon::LightGrass4 => TerrainSprite::LightGrass4,
            TerrainIcon::LightGrass5 => TerrainSprite::LightGrass5,
            TerrainIcon::LightGrass6 => TerrainSprite::LightGrass6,
            TerrainIcon::Savanah1 => TerrainSprite::Savanah1,
            TerrainIcon::Savanah2 => TerrainSprite::Savanah2,
            TerrainIcon::Savanah3 => TerrainSprite::Savanah3,
            TerrainIcon::Savanah4 => TerrainSprite::Savanah4,
            TerrainIcon::VeryLightGrass1 => TerrainSprite::VeryLightGrass1,
            TerrainIcon::VeryLightGrass2 => TerrainSprite::VeryLightGrass2,
            TerrainIcon::VeryLightGrass3 => TerrainSprite::VeryLightGrass3,
            TerrainIcon::VeryLightGrass4 => TerrainSprite::VeryLightGrass4,
            TerrainIcon::VeryLightGrass5 => TerrainSprite::VeryLightGrass5,
            TerrainIcon::Water1 => TerrainSprite::Water1,
            TerrainIcon::Water2 => TerrainSprite::Water2,
            TerrainIcon::Water3 => TerrainSprite::Water3,
            TerrainIcon::Water4 => TerrainSprite::Water4,
            TerrainIcon::Water5 => TerrainSprite::Water5,
            TerrainIcon::Water6 => TerrainSprite::Water6,
            TerrainIcon::Water7 => TerrainSprite::Water7,
            TerrainIcon::Water8 => TerrainSprite::Water8,
            TerrainIcon::Water9 => TerrainSprite::Water9,
            TerrainIcon::Water10 => TerrainSprite::Water10,
            TerrainIcon::Water11 => TerrainSprite::Water11,
            TerrainIcon::Water12 => TerrainSprite::Water12,
            TerrainIcon::Water13 => TerrainSprite::Water13,
            TerrainIcon::Water14 => TerrainSprite::Water14,
        }
    }
}

#[cfg(test)]
mod fence_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TerrainIcon::DarkGrass1, TerrainSprite::DarkGrass1)]
    #[case(TerrainIcon::DarkGrass2, TerrainSprite::DarkGrass2)]
    #[case(TerrainIcon::DarkGrass3, TerrainSprite::DarkGrass3)]
    #[case(TerrainIcon::DarkGrass4, TerrainSprite::DarkGrass4)]
    #[case(TerrainIcon::DarkGrass5, TerrainSprite::DarkGrass5)]
    #[case(TerrainIcon::DarkGrass6, TerrainSprite::DarkGrass6)]
    #[case(TerrainIcon::DarkGrass7, TerrainSprite::DarkGrass7)]
    #[case(TerrainIcon::DarkGrass8, TerrainSprite::DarkGrass8)]
    #[case(TerrainIcon::DarkGrass9, TerrainSprite::DarkGrass9)]
    #[case(TerrainIcon::Grass1, TerrainSprite::Grass1)]
    #[case(TerrainIcon::Grass2, TerrainSprite::Grass2)]
    #[case(TerrainIcon::Grass3, TerrainSprite::Grass3)]
    #[case(TerrainIcon::Grass4, TerrainSprite::Grass4)]
    #[case(TerrainIcon::Grass5, TerrainSprite::Grass5)]
    #[case(TerrainIcon::Grass6, TerrainSprite::Grass6)]
    #[case(TerrainIcon::LightGrass1, TerrainSprite::LightGrass1)]
    #[case(TerrainIcon::LightGrass2, TerrainSprite::LightGrass2)]
    #[case(TerrainIcon::LightGrass3, TerrainSprite::LightGrass3)]
    #[case(TerrainIcon::LightGrass4, TerrainSprite::LightGrass4)]
    #[case(TerrainIcon::LightGrass5, TerrainSprite::LightGrass5)]
    #[case(TerrainIcon::LightGrass6, TerrainSprite::LightGrass6)]
    #[case(TerrainIcon::Savanah1, TerrainSprite::Savanah1)]
    #[case(TerrainIcon::Savanah2, TerrainSprite::Savanah2)]
    #[case(TerrainIcon::Savanah3, TerrainSprite::Savanah3)]
    #[case(TerrainIcon::Savanah4, TerrainSprite::Savanah4)]
    #[case(TerrainIcon::VeryLightGrass1, TerrainSprite::VeryLightGrass1)]
    #[case(TerrainIcon::VeryLightGrass2, TerrainSprite::VeryLightGrass2)]
    #[case(TerrainIcon::VeryLightGrass3, TerrainSprite::VeryLightGrass3)]
    #[case(TerrainIcon::VeryLightGrass4, TerrainSprite::VeryLightGrass4)]
    #[case(TerrainIcon::VeryLightGrass5, TerrainSprite::VeryLightGrass5)]
    #[case(TerrainIcon::Water1, TerrainSprite::Water1)]
    #[case(TerrainIcon::Water2, TerrainSprite::Water2)]
    #[case(TerrainIcon::Water3, TerrainSprite::Water3)]
    #[case(TerrainIcon::Water4, TerrainSprite::Water4)]
    #[case(TerrainIcon::Water5, TerrainSprite::Water5)]
    #[case(TerrainIcon::Water6, TerrainSprite::Water6)]
    #[case(TerrainIcon::Water7, TerrainSprite::Water7)]
    #[case(TerrainIcon::Water8, TerrainSprite::Water8)]
    #[case(TerrainIcon::Water9, TerrainSprite::Water9)]
    #[case(TerrainIcon::Water10, TerrainSprite::Water10)]
    #[case(TerrainIcon::Water11, TerrainSprite::Water11)]
    #[case(TerrainIcon::Water12, TerrainSprite::Water12)]
    #[case(TerrainIcon::Water13, TerrainSprite::Water13)]
    #[case(TerrainIcon::Water14, TerrainSprite::Water14)]
    fn convert_from(#[case] icon: TerrainIcon, #[case] expected_sprite: TerrainSprite) {
        // when
        let actual_sprite = TerrainSprite::convert_from(icon);

        // then
        assert_eq!(expected_sprite, actual_sprite);
    }
}
