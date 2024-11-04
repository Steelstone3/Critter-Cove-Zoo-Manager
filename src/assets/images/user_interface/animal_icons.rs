use std::fmt::Display;

pub enum AnimalIcon {
    Boar,
    Chicken,
    Cow,
    Crab,
    Dog,
    Fox,
    Frog,
    Goat,
    Goose,
    Gorilla,
    Monkey,
    Moose,
    Pig,
    Porcupine,
    Sheep,
    Skunk,
    Toad,
    Turtle,
    Wolf,
}

impl Display for AnimalIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimalIcon::Boar => {
                write!(f, "images/user_interface/animal_menu/boar_icon.png")
            }
            AnimalIcon::Chicken => {
                write!(f, "images/user_interface/animal_menu/chicken_icon.png")
            }
            AnimalIcon::Cow => {
                write!(f, "images/user_interface/animal_menu/cow_icon.png")
            }
            AnimalIcon::Crab => {
                write!(f, "images/user_interface/animal_menu/crab_icon.png")
            }
            AnimalIcon::Dog => {
                write!(f, "images/user_interface/animal_menu/dog_icon.png")
            }
            AnimalIcon::Fox => {
                write!(f, "images/user_interface/animal_menu/fox_icon.png")
            }
            AnimalIcon::Frog => {
                write!(f, "images/user_interface/animal_menu/frog_icon.png")
            }
            AnimalIcon::Goat => {
                write!(f, "images/user_interface/animal_menu/goat_icon.png")
            }
            AnimalIcon::Goose => {
                write!(f, "images/user_interface/animal_menu/goose_icon.png")
            }
            AnimalIcon::Gorilla => {
                write!(f, "images/user_interface/animal_menu/gorilla_icon.png")
            }
            AnimalIcon::Monkey => {
                write!(f, "images/user_interface/animal_menu/monkey_icon.png")
            }
            AnimalIcon::Moose => {
                write!(f, "images/user_interface/animal_menu/moose_icon.png")
            }
            AnimalIcon::Pig => {
                write!(f, "images/user_interface/animal_menu/pig_icon.png")
            }
            AnimalIcon::Porcupine => {
                write!(f, "images/user_interface/animal_menu/porcupine_icon.png")
            }
            AnimalIcon::Sheep => {
                write!(f, "images/user_interface/animal_menu/sheep_icon.png")
            }
            AnimalIcon::Skunk => {
                write!(f, "images/user_interface/animal_menu/skunk_icon.png")
            }
            AnimalIcon::Toad => {
                write!(f, "images/user_interface/animal_menu/toad_icon.png")
            }
            AnimalIcon::Turtle => {
                write!(f, "images/user_interface/animal_menu/turtle_icon.png")
            }
            AnimalIcon::Wolf => {
                write!(f, "images/user_interface/animal_menu/wolf_icon.png")
            }
        }
    }
}
