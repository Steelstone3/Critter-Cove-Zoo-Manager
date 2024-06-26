use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum WorldRock {
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

impl Display for WorldRock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldRock::IceRock1 => write!(f, "images/world/rocks/ice_rock_1.png"),
            WorldRock::IceRock2 => write!(f, "images/world/rocks/ice_rock_2.png"),
            WorldRock::IceRock3 => write!(f, "images/world/rocks/ice_rock_3.png"),
            WorldRock::IceRock4 => write!(f, "images/world/rocks/ice_rock_4.png"),
            WorldRock::IceRock5 => write!(f, "images/world/rocks/ice_rock_5.png"),
            WorldRock::IceRock6 => write!(f, "images/world/rocks/ice_rock_6.png"),
            WorldRock::Rock1 => write!(f, "images/world/rocks/rock_1.png"),
            WorldRock::Rock2 => write!(f, "images/world/rocks/rock_2.png"),
            WorldRock::Rock3 => write!(f, "images/world/rocks/rock_3.png"),
            WorldRock::Rock4 => write!(f, "images/world/rocks/rock_4.png"),
            WorldRock::Rock5 => write!(f, "images/world/rocks/rock_5.png"),
            WorldRock::Rock6 => write!(f, "images/world/rocks/rock_6.png"),
            WorldRock::Rock7 => write!(f, "images/world/rocks/rock_7.png"),
            WorldRock::Rock8 => write!(f, "images/world/rocks/rock_8.png"),
            WorldRock::Rock9 => write!(f, "images/world/rocks/rock_9.png"),
            WorldRock::Rock10 => write!(f, "images/world/rocks/rock_10.png"),
            WorldRock::Rock11 => write!(f, "images/world/rocks/rock_11.png"),
            WorldRock::Rock12 => write!(f, "images/world/rocks/rock_12.png"),
            WorldRock::Rock13 => write!(f, "images/world/rocks/rock_13.png"),
            WorldRock::Rock14 => write!(f, "images/world/rocks/rock_14.png"),
            WorldRock::Rock15 => write!(f, "images/world/rocks/rock_15.png"),
            WorldRock::Rock16 => write!(f, "images/world/rocks/rock_16.png"),
            WorldRock::Rock17 => write!(f, "images/world/rocks/rock_17.png"),
            WorldRock::WaterRock1 => write!(f, "images/world/rocks/water_rock_1.png"),
            WorldRock::WaterRock2 => write!(f, "images/world/rocks/water_rock_2.png"),
            WorldRock::WaterRock3 => write!(f, "images/world/rocks/water_rock_3.png"),
            WorldRock::WaterRock4 => write!(f, "images/world/rocks/water_rock_4.png"),
            WorldRock::WaterRock5 => write!(f, "images/world/rocks/water_rock_5.png"),
            WorldRock::WaterRock6 => write!(f, "images/world/rocks/water_rock_6.png"),
            WorldRock::None => write!(f, ""), // No asset selected
        }
    }
}
