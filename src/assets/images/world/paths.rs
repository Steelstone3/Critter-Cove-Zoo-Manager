use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum WorldPath {
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

impl Display for WorldPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldPath::Path1 => write!(f, "images/world/paths/path_1.png"),
            WorldPath::Path2 => write!(f, "images/world/paths/path_2.png"),
            WorldPath::Path3 => write!(f, "images/world/paths/path_3.png"),
            WorldPath::Path4 => write!(f, "images/world/paths/path_4.png"),
            WorldPath::Path5 => write!(f, "images/world/paths/path_5.png"),
            WorldPath::Path6 => write!(f, "images/world/paths/path_6.png"),
            WorldPath::Path7 => write!(f, "images/world/paths/path_7.png"),
            WorldPath::Path8 => write!(f, "images/world/paths/path_8.png"),
            WorldPath::Path9 => write!(f, "images/world/paths/path_9.png"),
            WorldPath::Path10 => write!(f, "images/world/paths/path_10.png"),
            WorldPath::Path11 => write!(f, "images/world/paths/path_11.png"),
            WorldPath::Path12 => write!(f, "images/world/paths/path_12.png"),
            WorldPath::Path13 => write!(f, "images/world/paths/path_13.png"),
            WorldPath::Path14 => write!(f, "images/world/paths/path_14.png"),
            WorldPath::Path15 => write!(f, "images/world/paths/path_15.png"),
            WorldPath::Path16 => write!(f, "images/world/paths/path_16.png"),
            WorldPath::Path17 => write!(f, "images/world/paths/path_17.png"),
            WorldPath::Path18 => write!(f, "images/world/paths/path_18.png"),
            WorldPath::Path19 => write!(f, "images/world/paths/path_19.png"),
            WorldPath::Path20 => write!(f, "images/world/paths/path_20.png"),
            WorldPath::Path21 => write!(f, "images/world/paths/path_21.png"),
            WorldPath::Path22 => write!(f, "images/world/paths/path_22.png"),
            WorldPath::Path23 => write!(f, "images/world/paths/path_23.png"),
            WorldPath::None => write!(f, ""),
        }
    }
}
