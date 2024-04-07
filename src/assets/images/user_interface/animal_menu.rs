use std::fmt::Display;

#[allow(dead_code)]
pub enum AnimalUserInterface {
    MooseIcon,
    SelectedMooseIcon,
}

impl Display for AnimalUserInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimalUserInterface::MooseIcon => {
                write!(f, "images/user_interface/animal_menu/moose_icon.png")
            }
            AnimalUserInterface::SelectedMooseIcon => {
                write!(
                    f,
                    "images/user_interface/animal_menu/selected_moose_icon.png"
                )
            }
        }
    }
}
