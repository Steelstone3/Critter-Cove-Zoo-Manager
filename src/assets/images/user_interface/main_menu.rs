use std::fmt::Display;

pub enum MainMenuIcon {
    Animals,
    Fences,
    Terrain,
    Trees,
    Rocks,
    Paths,
}

impl Display for MainMenuIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuIcon::Animals => {
                write!(f, "images/user_interface/main_menu/animals_icon.png")
            }
            MainMenuIcon::Fences => {
                write!(f, "images/user_interface/main_menu/fences_icon.png")
            }
            MainMenuIcon::Terrain => {
                write!(f, "images/user_interface/main_menu/terrains_icon.png")
            }
            MainMenuIcon::Trees => {
                write!(f, "images/user_interface/main_menu/trees_icon.png")
            }
            MainMenuIcon::Rocks => {
                write!(f, "images/user_interface/main_menu/rocks_icon.png")
            }
            MainMenuIcon::Paths => {
                write!(f, "images/user_interface/main_menu/paths_icon.png")
            }
        }
    }
}
