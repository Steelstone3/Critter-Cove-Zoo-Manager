use std::fmt::Display;

pub enum MainMenuUserInterface {
    IconAnimals,
    // SelectedIconAnimals,
}

impl Display for MainMenuUserInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MainMenuUserInterface::IconAnimals => {
                write!(f, "images/user_interface/main_menu/animals_icon.png")
            } // MainMenuUserInterface::SelectedIconAnimals => {
              //     write!(
              //         f,
              //         "images/user_interface/main_menu/selected_animals_icon.png"
              //     )
              // }
        }
    }
}
