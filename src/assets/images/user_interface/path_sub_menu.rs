use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum PathSubMenu {
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
}

impl Display for PathSubMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathSubMenu::Path1 => {
                write!(f, "images/user_interface/path_menu/path_1_icon.png")
            }
            PathSubMenu::Path2 => {
                write!(f, "images/user_interface/path_menu/path_2_icon.png")
            }
            PathSubMenu::Path3 => {
                write!(f, "images/user_interface/path_menu/path_3_icon.png")
            }
            PathSubMenu::Path4 => {
                write!(f, "images/user_interface/path_menu/path_4_icon.png")
            }
            PathSubMenu::Path5 => {
                write!(f, "images/user_interface/path_menu/path_5_icon.png")
            }
            PathSubMenu::Path6 => {
                write!(f, "images/user_interface/path_menu/path_6_icon.png")
            }
            PathSubMenu::Path7 => {
                write!(f, "images/user_interface/path_menu/path_7_icon.png")
            }
            PathSubMenu::Path8 => {
                write!(f, "images/user_interface/path_menu/path_8_icon.png")
            }
            PathSubMenu::Path9 => {
                write!(f, "images/user_interface/path_menu/path_9_icon.png")
            }
            PathSubMenu::Path10 => {
                write!(f, "images/user_interface/path_menu/path_10_icon.png")
            }
            PathSubMenu::Path11 => {
                write!(f, "images/user_interface/path_menu/path_11_icon.png")
            }
            PathSubMenu::Path12 => {
                write!(f, "images/user_interface/path_menu/path_12_icon.png")
            }
            PathSubMenu::Path13 => {
                write!(f, "images/user_interface/path_menu/path_13_icon.png")
            }
            PathSubMenu::Path14 => {
                write!(f, "images/user_interface/path_menu/path_14_icon.png")
            }
            PathSubMenu::Path15 => {
                write!(f, "images/user_interface/path_menu/path_15_icon.png")
            }
            PathSubMenu::Path16 => {
                write!(f, "images/user_interface/path_menu/path_16_icon.png")
            }
            PathSubMenu::Path17 => {
                write!(f, "images/user_interface/path_menu/path_17_icon.png")
            }
            PathSubMenu::Path18 => {
                write!(f, "images/user_interface/path_menu/path_18_icon.png")
            }
            PathSubMenu::Path19 => {
                write!(f, "images/user_interface/path_menu/path_19_icon.png")
            }
            PathSubMenu::Path20 => {
                write!(f, "images/user_interface/path_menu/path_20_icon.png")
            }
            PathSubMenu::Path21 => {
                write!(f, "images/user_interface/path_menu/path_21_icon.png")
            }
            PathSubMenu::Path22 => {
                write!(f, "images/user_interface/path_menu/path_22_icon.png")
            }
            PathSubMenu::Path23 => {
                write!(f, "images/user_interface/path_menu/path_23_icon.png")
            }
        }
    }
}
