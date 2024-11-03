use std::fmt::Display;

pub enum TreeIcon {
    Bush1,
    Bush2,
    TallGrass1,
    TallGrass2,
    TallGrass3,
    Tree1,
    Tree2,
    Tree3,
    Tree4,
}

impl Display for TreeIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeIcon::Bush1 => {
                write!(f, "images/user_interface/tree_menu/bush_1_icon.png")
            }
            TreeIcon::Bush2 => {
                write!(f, "images/user_interface/tree_menu/bush_2_icon.png")
            }
            TreeIcon::TallGrass1 => {
                write!(f, "images/user_interface/tree_menu/tall_grass_1_icon.png")
            }
            TreeIcon::TallGrass2 => {
                write!(f, "images/user_interface/tree_menu/tall_grass_2_icon.png")
            }
            TreeIcon::TallGrass3 => {
                write!(f, "images/user_interface/tree_menu/tall_grass_3_icon.png")
            }
            TreeIcon::Tree1 => {
                write!(f, "images/user_interface/tree_menu/tree_1_icon.png")
            }
            TreeIcon::Tree2 => {
                write!(f, "images/user_interface/tree_menu/tree_2_icon.png")
            }
            TreeIcon::Tree3 => {
                write!(f, "images/user_interface/tree_menu/tree_3_icon.png")
            }
            TreeIcon::Tree4 => {
                write!(f, "images/user_interface/tree_menu/tree_4_icon.png")
            }
        }
    }
}
