use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum WorldTree {
    Tree1,
    Tree2,
    Tree3,
    Tree4,
    None,
}

impl Display for WorldTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldTree::Tree1 => write!(f, "images/world/trees/tree_1.png"),
            WorldTree::Tree2 => write!(f, "images/world/trees/tree_2.png"),
            WorldTree::Tree3 => write!(f, "images/world/trees/tree_3.png"),
            WorldTree::Tree4 => write!(f, "images/world/trees/tree_4.png"),
            WorldTree::None => write!(f, ""),
        }
    }
}
