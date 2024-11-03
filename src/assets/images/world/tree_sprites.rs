use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum TreeSprite {
    Bush1,
    Bush2,
    TallGrass1,
    TallGrass2,
    TallGrass3,
    Tree1,
    Tree2,
    Tree3,
    Tree4,
    None,
}

impl Display for TreeSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeSprite::Bush1 => write!(f, "images/world/trees/bush_1.png"),
            TreeSprite::Bush2 => write!(f, "images/world/trees/bush_2.png"),
            TreeSprite::TallGrass1 => write!(f, "images/world/trees/tall_grass_1.png"),
            TreeSprite::TallGrass2 => write!(f, "images/world/trees/tall_grass_2.png"),
            TreeSprite::TallGrass3 => write!(f, "images/world/trees/tall_grass_3.png"),
            TreeSprite::Tree1 => write!(f, "images/world/trees/tree_1.png"),
            TreeSprite::Tree2 => write!(f, "images/world/trees/tree_2.png"),
            TreeSprite::Tree3 => write!(f, "images/world/trees/tree_3.png"),
            TreeSprite::Tree4 => write!(f, "images/world/trees/tree_4.png"),
            TreeSprite::None => write!(f, ""),
        }
    }
}
