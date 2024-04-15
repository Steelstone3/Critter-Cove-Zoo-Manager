use std::fmt::Display;

pub enum MainMenuUserInterface {
    Animals,
    Rocks,
}

impl Display for MainMenuUserInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuUserInterface::Animals => {
                write!(f, "images/user_interface/main_menu/animals_icon.png")
            }
            MainMenuUserInterface::Rocks => {
                write!(f, "images/user_interface/main_menu/rocks_icon.png")
            }
        }
    }
}
