use std::fmt::Display;

use crate::assets::images::user_interface::rock_icons::RockIcon;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum RockSprite {
    IceRock1,
    IceRock2,
    IceRock3,
    IceRock4,
    IceRock5,
    IceRock6,
    Rock1,
    Rock2,
    Rock3,
    Rock4,
    Rock5,
    Rock6,
    Rock7,
    Rock8,
    Rock9,
    Rock10,
    Rock11,
    Rock12,
    Rock13,
    Rock14,
    Rock15,
    Rock16,
    Rock17,
    WaterRock1,
    WaterRock2,
    WaterRock3,
    WaterRock4,
    WaterRock5,
    WaterRock6,
    None,
}

impl Display for RockSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RockSprite::IceRock1 => write!(f, "images/world/rocks/ice_rock_1.png"),
            RockSprite::IceRock2 => write!(f, "images/world/rocks/ice_rock_2.png"),
            RockSprite::IceRock3 => write!(f, "images/world/rocks/ice_rock_3.png"),
            RockSprite::IceRock4 => write!(f, "images/world/rocks/ice_rock_4.png"),
            RockSprite::IceRock5 => write!(f, "images/world/rocks/ice_rock_5.png"),
            RockSprite::IceRock6 => write!(f, "images/world/rocks/ice_rock_6.png"),
            RockSprite::Rock1 => write!(f, "images/world/rocks/rock_1.png"),
            RockSprite::Rock2 => write!(f, "images/world/rocks/rock_2.png"),
            RockSprite::Rock3 => write!(f, "images/world/rocks/rock_3.png"),
            RockSprite::Rock4 => write!(f, "images/world/rocks/rock_4.png"),
            RockSprite::Rock5 => write!(f, "images/world/rocks/rock_5.png"),
            RockSprite::Rock6 => write!(f, "images/world/rocks/rock_6.png"),
            RockSprite::Rock7 => write!(f, "images/world/rocks/rock_7.png"),
            RockSprite::Rock8 => write!(f, "images/world/rocks/rock_8.png"),
            RockSprite::Rock9 => write!(f, "images/world/rocks/rock_9.png"),
            RockSprite::Rock10 => write!(f, "images/world/rocks/rock_10.png"),
            RockSprite::Rock11 => write!(f, "images/world/rocks/rock_11.png"),
            RockSprite::Rock12 => write!(f, "images/world/rocks/rock_12.png"),
            RockSprite::Rock13 => write!(f, "images/world/rocks/rock_13.png"),
            RockSprite::Rock14 => write!(f, "images/world/rocks/rock_14.png"),
            RockSprite::Rock15 => write!(f, "images/world/rocks/rock_15.png"),
            RockSprite::Rock16 => write!(f, "images/world/rocks/rock_16.png"),
            RockSprite::Rock17 => write!(f, "images/world/rocks/rock_17.png"),
            RockSprite::WaterRock1 => write!(f, "images/world/rocks/water_rock_1.png"),
            RockSprite::WaterRock2 => write!(f, "images/world/rocks/water_rock_2.png"),
            RockSprite::WaterRock3 => write!(f, "images/world/rocks/water_rock_3.png"),
            RockSprite::WaterRock4 => write!(f, "images/world/rocks/water_rock_4.png"),
            RockSprite::WaterRock5 => write!(f, "images/world/rocks/water_rock_5.png"),
            RockSprite::WaterRock6 => write!(f, "images/world/rocks/water_rock_6.png"),
            RockSprite::None => write!(f, ""), // No asset selected
        }
    }
}

impl RockSprite {
    pub fn convert_from(icon: RockIcon) -> Self {
        match icon {
            RockIcon::IceRock1 => RockSprite::IceRock1,
            RockIcon::IceRock2 => RockSprite::IceRock2,
            RockIcon::IceRock3 => RockSprite::IceRock3,
            RockIcon::IceRock4 => RockSprite::IceRock4,
            RockIcon::IceRock5 => RockSprite::IceRock5,
            RockIcon::IceRock6 => RockSprite::IceRock6,
            RockIcon::Rock1 => RockSprite::Rock1,
            RockIcon::Rock2 => RockSprite::Rock2,
            RockIcon::Rock3 => RockSprite::Rock3,
            RockIcon::Rock4 => RockSprite::Rock4,
            RockIcon::Rock5 => RockSprite::Rock5,
            RockIcon::Rock6 => RockSprite::Rock6,
            RockIcon::Rock7 => RockSprite::Rock7,
            RockIcon::Rock8 => RockSprite::Rock8,
            RockIcon::Rock9 => RockSprite::Rock9,
            RockIcon::Rock10 => RockSprite::Rock10,
            RockIcon::Rock11 => RockSprite::Rock11,
            RockIcon::Rock12 => RockSprite::Rock12,
            RockIcon::Rock13 => RockSprite::Rock13,
            RockIcon::Rock14 => RockSprite::Rock14,
            RockIcon::Rock15 => RockSprite::Rock15,
            RockIcon::Rock16 => RockSprite::Rock16,
            RockIcon::Rock17 => RockSprite::Rock17,
            RockIcon::WaterRock1 => RockSprite::WaterRock1,
            RockIcon::WaterRock2 => RockSprite::WaterRock2,
            RockIcon::WaterRock3 => RockSprite::WaterRock3,
            RockIcon::WaterRock4 => RockSprite::WaterRock4,
            RockIcon::WaterRock5 => RockSprite::WaterRock5,
            RockIcon::WaterRock6 => RockSprite::WaterRock6,
        }
    }
}

#[cfg(test)]
mod rock_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(RockIcon::IceRock1, RockSprite::IceRock1)]
    #[case(RockIcon::IceRock2, RockSprite::IceRock2)]
    #[case(RockIcon::IceRock3, RockSprite::IceRock3)]
    #[case(RockIcon::IceRock4, RockSprite::IceRock4)]
    #[case(RockIcon::IceRock5, RockSprite::IceRock5)]
    #[case(RockIcon::IceRock6, RockSprite::IceRock6)]
    #[case(RockIcon::Rock1, RockSprite::Rock1)]
    #[case(RockIcon::Rock2, RockSprite::Rock2)]
    #[case(RockIcon::Rock3, RockSprite::Rock3)]
    #[case(RockIcon::Rock4, RockSprite::Rock4)]
    #[case(RockIcon::Rock5, RockSprite::Rock5)]
    #[case(RockIcon::Rock6, RockSprite::Rock6)]
    #[case(RockIcon::Rock7, RockSprite::Rock7)]
    #[case(RockIcon::Rock8, RockSprite::Rock8)]
    #[case(RockIcon::Rock9, RockSprite::Rock9)]
    #[case(RockIcon::Rock10, RockSprite::Rock10)]
    #[case(RockIcon::Rock11, RockSprite::Rock11)]
    #[case(RockIcon::Rock12, RockSprite::Rock12)]
    #[case(RockIcon::Rock13, RockSprite::Rock13)]
    #[case(RockIcon::Rock14, RockSprite::Rock14)]
    #[case(RockIcon::Rock15, RockSprite::Rock15)]
    #[case(RockIcon::Rock16, RockSprite::Rock16)]
    #[case(RockIcon::Rock17, RockSprite::Rock17)]
    #[case(RockIcon::WaterRock1, RockSprite::WaterRock1)]
    #[case(RockIcon::WaterRock2, RockSprite::WaterRock2)]
    #[case(RockIcon::WaterRock3, RockSprite::WaterRock3)]
    #[case(RockIcon::WaterRock4, RockSprite::WaterRock4)]
    #[case(RockIcon::WaterRock5, RockSprite::WaterRock5)]
    #[case(RockIcon::WaterRock6, RockSprite::WaterRock6)]
    fn convert_from(#[case] icon: RockIcon, #[case] expected_sprite: RockSprite) {
        // when
        let actual_sprite = RockSprite::convert_from(icon);

        // then
        assert_eq!(expected_sprite, actual_sprite);
    }
}
