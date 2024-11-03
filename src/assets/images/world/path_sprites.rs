use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum PathSprite {
    Path1,
    Path2,
    Path3,
    Path4,
    Path5,
    Path6,
    Path7,
    Path8,
    Path9,
    Path10,
    Path11,
    Path12,
    Path13,
    Path14,
    Path15,
    Path16,
    Path17,
    Path18,
    Path19,
    Path20,
    Path21,
    Path22,
    Path23,
    None,
}

impl Display for PathSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathSprite::Path1 => write!(f, "images/world/paths/path_1.png"),
            PathSprite::Path2 => write!(f, "images/world/paths/path_2.png"),
            PathSprite::Path3 => write!(f, "images/world/paths/path_3.png"),
            PathSprite::Path4 => write!(f, "images/world/paths/path_4.png"),
            PathSprite::Path5 => write!(f, "images/world/paths/path_5.png"),
            PathSprite::Path6 => write!(f, "images/world/paths/path_6.png"),
            PathSprite::Path7 => write!(f, "images/world/paths/path_7.png"),
            PathSprite::Path8 => write!(f, "images/world/paths/path_8.png"),
            PathSprite::Path9 => write!(f, "images/world/paths/path_9.png"),
            PathSprite::Path10 => write!(f, "images/world/paths/path_10.png"),
            PathSprite::Path11 => write!(f, "images/world/paths/path_11.png"),
            PathSprite::Path12 => write!(f, "images/world/paths/path_12.png"),
            PathSprite::Path13 => write!(f, "images/world/paths/path_13.png"),
            PathSprite::Path14 => write!(f, "images/world/paths/path_14.png"),
            PathSprite::Path15 => write!(f, "images/world/paths/path_15.png"),
            PathSprite::Path16 => write!(f, "images/world/paths/path_16.png"),
            PathSprite::Path17 => write!(f, "images/world/paths/path_17.png"),
            PathSprite::Path18 => write!(f, "images/world/paths/path_18.png"),
            PathSprite::Path19 => write!(f, "images/world/paths/path_19.png"),
            PathSprite::Path20 => write!(f, "images/world/paths/path_20.png"),
            PathSprite::Path21 => write!(f, "images/world/paths/path_21.png"),
            PathSprite::Path22 => write!(f, "images/world/paths/path_22.png"),
            PathSprite::Path23 => write!(f, "images/world/paths/path_23.png"),
            PathSprite::None => write!(f, ""),
        }
    }
}
