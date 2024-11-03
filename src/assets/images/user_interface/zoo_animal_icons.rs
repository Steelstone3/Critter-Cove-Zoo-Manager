use std::fmt::Display;

pub enum ZooAnimalIcon {
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

impl Display for ZooAnimalIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZooAnimalIcon::Boar => {
                write!(f, "images/user_interface/animal_menu/boar_icon.png")
            }
            ZooAnimalIcon::Chicken => {
                write!(f, "images/user_interface/animal_menu/chicken_icon.png")
            }
            ZooAnimalIcon::Cow => {
                write!(f, "images/user_interface/animal_menu/cow_icon.png")
            }
            ZooAnimalIcon::Crab => {
                write!(f, "images/user_interface/animal_menu/crab_icon.png")
            }
            ZooAnimalIcon::Dog => {
                write!(f, "images/user_interface/animal_menu/dog_icon.png")
            }
            ZooAnimalIcon::Fox => {
                write!(f, "images/user_interface/animal_menu/fox_icon.png")
            }
            ZooAnimalIcon::Frog => {
                write!(f, "images/user_interface/animal_menu/frog_icon.png")
            }
            ZooAnimalIcon::Goat => {
                write!(f, "images/user_interface/animal_menu/goat_icon.png")
            }
            ZooAnimalIcon::Goose => {
                write!(f, "images/user_interface/animal_menu/goose_icon.png")
            }
            ZooAnimalIcon::Gorilla => {
                write!(f, "images/user_interface/animal_menu/gorilla_icon.png")
            }
            ZooAnimalIcon::Monkey => {
                write!(f, "images/user_interface/animal_menu/monkey_icon.png")
            }
            ZooAnimalIcon::Moose => {
                write!(f, "images/user_interface/animal_menu/moose_icon.png")
            }
            ZooAnimalIcon::Pig => {
                write!(f, "images/user_interface/animal_menu/pig_icon.png")
            }
            ZooAnimalIcon::Porcupine => {
                write!(f, "images/user_interface/animal_menu/porcupine_icon.png")
            }
            ZooAnimalIcon::Sheep => {
                write!(f, "images/user_interface/animal_menu/sheep_icon.png")
            }
            ZooAnimalIcon::Skunk => {
                write!(f, "images/user_interface/animal_menu/skunk_icon.png")
            }
            ZooAnimalIcon::Toad => {
                write!(f, "images/user_interface/animal_menu/toad_icon.png")
            }
            ZooAnimalIcon::Turtle => {
                write!(f, "images/user_interface/animal_menu/turtle_icon.png")
            }
            ZooAnimalIcon::Wolf => {
                write!(f, "images/user_interface/animal_menu/wolf_icon.png")
            }
        }
    }
}
