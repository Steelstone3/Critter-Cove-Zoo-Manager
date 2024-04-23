use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum WorldTree {
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

impl Display for WorldTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldTree::Bush1 => write!(f, "images/world/trees/bush_1.png"),
            WorldTree::Bush2 => write!(f, "images/world/trees/bush_2.png"),
            WorldTree::TallGrass1 => write!(f, "images/world/trees/tall_grass_1.png"),
            WorldTree::TallGrass2 => write!(f, "images/world/trees/tall_grass_2.png"),
            WorldTree::TallGrass3 => write!(f, "images/world/trees/tall_grass_3.png"),
            WorldTree::Tree1 => write!(f, "images/world/trees/tree_1.png"),
            WorldTree::Tree2 => write!(f, "images/world/trees/tree_2.png"),
            WorldTree::Tree3 => write!(f, "images/world/trees/tree_3.png"),
            WorldTree::Tree4 => write!(f, "images/world/trees/tree_4.png"),
            WorldTree::None => write!(f, ""),
        }
    }
}
