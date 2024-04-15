use std::fmt::Display;

#[allow(dead_code)]
pub enum RockSubMenu {
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

impl Display for RockSubMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RockSubMenu::IceRock1 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_1_icon.png")
            }
            RockSubMenu::IceRock2 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_2_icon.png")
            }
            RockSubMenu::IceRock3 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_3_icon.png")
            }
            RockSubMenu::IceRock4 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_4_icon.png")
            }
            RockSubMenu::IceRock5 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_5_icon.png")
            }
            RockSubMenu::IceRock6 => {
                write!(f, "images/user_interface/rock_menu/ice_rock_6_icon.png")
            }
            RockSubMenu::Rock1 => write!(f, "images/user_interface/rock_menu/rock_1_icon.png"),
            RockSubMenu::Rock2 => write!(f, "images/user_interface/rock_menu/rock_2_icon.png"),
            RockSubMenu::Rock3 => write!(f, "images/user_interface/rock_menu/rock_3_icon.png"),
            RockSubMenu::Rock4 => write!(f, "images/user_interface/rock_menu/rock_4_icon.png"),
            RockSubMenu::Rock5 => write!(f, "images/user_interface/rock_menu/rock_5_icon.png"),
            RockSubMenu::Rock6 => write!(f, "images/user_interface/rock_menu/rock_6_icon.png"),
            RockSubMenu::Rock7 => write!(f, "images/user_interface/rock_menu/rock_7_icon.png"),
            RockSubMenu::Rock8 => write!(f, "images/user_interface/rock_menu/rock_8_icon.png"),
            RockSubMenu::Rock9 => write!(f, "images/user_interface/rock_menu/rock_9_icon.png"),
            RockSubMenu::Rock10 => write!(f, "images/user_interface/rock_menu/rock_10_icon.png"),
            RockSubMenu::Rock11 => write!(f, "images/user_interface/rock_menu/rock_11_icon.png"),
            RockSubMenu::Rock12 => write!(f, "images/user_interface/rock_menu/rock_12_icon.png"),
            RockSubMenu::Rock13 => write!(f, "images/user_interface/rock_menu/rock_13_icon.png"),
            RockSubMenu::Rock14 => write!(f, "images/user_interface/rock_menu/rock_14_icon.png"),
            RockSubMenu::Rock15 => write!(f, "images/user_interface/rock_menu/rock_15_icon.png"),
            RockSubMenu::Rock16 => write!(f, "images/user_interface/rock_menu/rock_16_icon.png"),
            RockSubMenu::Rock17 => write!(f, "images/user_interface/rock_menu/rock_17_icon.png"),
            RockSubMenu::WaterRock1 => {
                write!(f, "images/user_interface/rock_menu/water_rock_1.png")
            }
            RockSubMenu::WaterRock2 => {
                write!(f, "images/user_interface/rock_menu/water_rock_2.png")
            }
            RockSubMenu::WaterRock3 => {
                write!(f, "images/user_interface/rock_menu/water_rock_3.png")
            }
            RockSubMenu::WaterRock4 => {
                write!(f, "images/user_interface/rock_menu/water_rock_4.png")
            }
            RockSubMenu::WaterRock5 => {
                write!(f, "images/user_interface/rock_menu/water_rock_5.png")
            }
            RockSubMenu::WaterRock6 => {
                write!(f, "images/user_interface/rock_menu/water_rock_6.png")
            }
        }
    }
}
