use std::fmt::Display;

pub enum TreeSubMenu {
    Tree1,
    Tree2,
    Tree3,
    Tree4,
}

impl Display for TreeSubMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
