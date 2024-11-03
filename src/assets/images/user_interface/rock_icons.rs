use std::fmt::Display;

pub enum RockIcon {
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
}

impl Display for RockIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RockIcon::IceRock1 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_1_icon.png")
            }
            RockIcon::IceRock2 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_2_icon.png")
            }
            RockIcon::IceRock3 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_3_icon.png")
            }
            RockIcon::IceRock4 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_4_icon.png")
            }
            RockIcon::IceRock5 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_5_icon.png")
            }
            RockIcon::IceRock6 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_6_icon.png")
            }
            RockIcon::Rock1 => write!(f, "images/user_interface/rock_menu/rock_1_icon.png"),
            RockIcon::Rock2 => write!(f, "images/user_interface/rock_menu/rock_2_icon.png"),
            RockIcon::Rock3 => write!(f, "images/user_interface/rock_menu/rock_3_icon.png"),
            RockIcon::Rock4 => write!(f, "images/user_interface/rock_menu/rock_4_icon.png"),
            RockIcon::Rock5 => write!(f, "images/user_interface/rock_menu/rock_5_icon.png"),
            RockIcon::Rock6 => write!(f, "images/user_interface/rock_menu/rock_6_icon.png"),
            RockIcon::Rock7 => write!(f, "images/user_interface/rock_menu/rock_7_icon.png"),
            RockIcon::Rock8 => write!(f, "images/user_interface/rock_menu/rock_8_icon.png"),
            RockIcon::Rock9 => write!(f, "images/user_interface/rock_menu/rock_9_icon.png"),
            RockIcon::Rock10 => write!(f, "images/user_interface/rock_menu/rock_10_icon.png"),
            RockIcon::Rock11 => write!(f, "images/user_interface/rock_menu/rock_11_icon.png"),
            RockIcon::Rock12 => write!(f, "images/user_interface/rock_menu/rock_12_icon.png"),
            RockIcon::Rock13 => write!(f, "images/user_interface/rock_menu/rock_13_icon.png"),
            RockIcon::Rock14 => write!(f, "images/user_interface/rock_menu/rock_14_icon.png"),
            RockIcon::Rock15 => write!(f, "images/user_interface/rock_menu/rock_15_icon.png"),
            RockIcon::Rock16 => write!(f, "images/user_interface/rock_menu/rock_16_icon.png"),
            RockIcon::Rock17 => write!(f, "images/user_interface/rock_menu/rock_17_icon.png"),
            RockIcon::WaterRock1 => {
                write!(f, "images/user_interface/rock_menu/water_rock_1_icon.png")
            }
            RockIcon::WaterRock2 => {
                write!(f, "images/user_interface/rock_menu/water_rock_2_icon.png")
            }
            RockIcon::WaterRock3 => {
                write!(f, "images/user_interface/rock_menu/water_rock_3_icon.png")
            }
            RockIcon::WaterRock4 => {
                write!(f, "images/user_interface/rock_menu/water_rock_4_icon.png")
            }
            RockIcon::WaterRock5 => {
                write!(f, "images/user_interface/rock_menu/water_rock_5_icon.png")
            }
            RockIcon::WaterRock6 => {
                write!(f, "images/user_interface/rock_menu/water_rock_6_icon.png")
            }
        }
    }
}
