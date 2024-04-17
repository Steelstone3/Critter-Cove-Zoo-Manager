use std::fmt::Display;

pub enum AnimalSubMenu {
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

impl Display for AnimalSubMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnimalSubMenu::Boar => {
                write!(f, "images/user_interface/animal_menu/boar_icon.png")
            }
            AnimalSubMenu::Chicken => {
                write!(f, "images/user_interface/animal_menu/chicken_icon.png")
            }
            AnimalSubMenu::Cow => {
                write!(f, "images/user_interface/animal_menu/cow_icon.png")
            }
            AnimalSubMenu::Crab => {
                write!(f, "images/user_interface/animal_menu/crab_icon.png")
            }
            AnimalSubMenu::Dog => {
                write!(f, "images/user_interface/animal_menu/dog_icon.png")
            }
            AnimalSubMenu::Fox => {
                write!(f, "images/user_interface/animal_menu/fox_icon.png")
            }
            AnimalSubMenu::Frog => {
                write!(f, "images/user_interface/animal_menu/frog_icon.png")
            }
            AnimalSubMenu::Goat => {
                write!(f, "images/user_interface/animal_menu/goat_icon.png")
            }
            AnimalSubMenu::Goose => {
                write!(f, "images/user_interface/animal_menu/goose_icon.png")
            }
            AnimalSubMenu::Gorilla => {
                write!(f, "images/user_interface/animal_menu/gorilla_icon.png")
            }
            AnimalSubMenu::Monkey => {
                write!(f, "images/user_interface/animal_menu/monkey_icon.png")
            }
            AnimalSubMenu::Moose => {
                write!(f, "images/user_interface/animal_menu/moose_icon.png")
            }
            AnimalSubMenu::Pig => {
                write!(f, "images/user_interface/animal_menu/pig_icon.png")
            }
            AnimalSubMenu::Porcupine => {
                write!(f, "images/user_interface/animal_menu/porcupine_icon.png")
            }
            AnimalSubMenu::Sheep => {
                write!(f, "images/user_interface/animal_menu/sheep_icon.png")
            }
            AnimalSubMenu::Skunk => {
                write!(f, "images/user_interface/animal_menu/skunk_icon.png")
            }
            AnimalSubMenu::Toad => {
                write!(f, "images/user_interface/animal_menu/toad_icon.png")
            }
            AnimalSubMenu::Turtle => {
                write!(f, "images/user_interface/animal_menu/turtle_icon.png")
            }
            AnimalSubMenu::Wolf => {
                write!(f, "images/user_interface/animal_menu/wolf_icon.png")
            }
        }
    }
}
