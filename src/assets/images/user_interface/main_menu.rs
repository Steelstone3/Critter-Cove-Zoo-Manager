use std::fmt::Display;

pub enum MainMenuUserInterface {
    Animals,
}

impl Display for MainMenuUserInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuUserInterface::Animals => {
                write!(f, "images/user_interface/main_menu/animals_icon.png")
            }
        }
    }
}
