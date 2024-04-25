use std::fmt::Display;

pub enum TreeSubMenu {
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

impl Display for TreeSubMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeSubMenu::Bush1 => {
                write!(f, "images/user_interface/tree_menu/bush_1_icon.png")
            }
            TreeSubMenu::Bush2 => {
                write!(f, "images/user_interface/tree_menu/bush_2_icon.png")
            }
            TreeSubMenu::TallGrass1 => {
                write!(f, "images/user_interface/tree_menu/tall_grass_1_icon.png")
            }
            TreeSubMenu::TallGrass2 => {
                write!(f, "images/user_interface/tree_menu/tall_grass_2_icon.png")
            }
            TreeSubMenu::TallGrass3 => {
                write!(f, "images/user_interface/tree_menu/tall_grass_3_icon.png")
            }
            TreeSubMenu::Tree1 => {
                write!(f, "images/user_interface/tree_menu/tree_1_icon.png")
            }
            TreeSubMenu::Tree2 => {
                write!(f, "images/user_interface/tree_menu/tree_2_icon.png")
            }
            TreeSubMenu::Tree3 => {
                write!(f, "images/user_interface/tree_menu/tree_3_icon.png")
            }
            TreeSubMenu::Tree4 => {
                write!(f, "images/user_interface/tree_menu/tree_4_icon.png")
            }
        }
    }
}
