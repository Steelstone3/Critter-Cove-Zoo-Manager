use std::fmt::Display;

pub enum UserInterface {
    IconAnimals,
}

impl Display for UserInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserInterface::IconAnimals => {
                write!(f, "images/user_interface/animals_icon.png")
            }
        }
    }
}
