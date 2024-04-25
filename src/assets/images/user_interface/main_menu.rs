use std::fmt::Display;

pub enum MainMenuUserInterface {
    Animals,
    Fences,
    Terrain,
    Trees,
    Rocks,
    Paths,
}

impl Display for MainMenuUserInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuUserInterface::Animals => {
                write!(f, "images/user_interface/main_menu/animals_icon.png")
            }
            MainMenuUserInterface::Fences => {
                write!(f, "images/user_interface/main_menu/fences_icon.png")
            }
            MainMenuUserInterface::Terrain => {
                write!(f, "images/user_interface/main_menu/terrains_icon.png")
            }
            MainMenuUserInterface::Trees => {
                write!(f, "images/user_interface/main_menu/trees_icon.png")
            }
            MainMenuUserInterface::Rocks => {
                write!(f, "images/user_interface/main_menu/rocks_icon.png")
            }
            MainMenuUserInterface::Paths => {
                write!(f, "images/user_interface/main_menu/paths_icon.png")
            }
        }
    }
}
