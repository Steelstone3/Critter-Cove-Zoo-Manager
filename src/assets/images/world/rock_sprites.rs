use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
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
