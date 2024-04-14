use std::fmt::Display;

pub enum AnimalSubMenu {
    MooseIcon,
    // SelectedMooseIcon,
}

impl Display for AnimalSubMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimalSubMenu::MooseIcon => {
                write!(f, "images/user_interface/animal_menu/moose_icon.png")
            } // AnimalUserInterface::SelectedMooseIcon => {
              //     write!(
              //         f,
              //         "images/user_interface/animal_menu/selected_moose_icon.png"
              //     )
              // }
        }
    }
}
