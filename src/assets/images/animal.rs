use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen, Clone, Copy)]
pub enum ZooAnimal {
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
    None,
}

impl Display for ZooAnimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZooAnimal::Boar => {
                write!(f, "images/animals/zoo/boar.png")
            }
            ZooAnimal::Chicken => {
                write!(f, "images/animals/zoo/chicken.png")
            }
            ZooAnimal::Cow => {
                write!(f, "images/animals/zoo/cow.png")
            }
            ZooAnimal::Crab => {
                write!(f, "images/animals/zoo/crab.png")
            }
            ZooAnimal::Dog => {
                write!(f, "images/animals/zoo/dog.png")
            }
            ZooAnimal::Fox => {
                write!(f, "images/animals/zoo/fox.png")
            }
            ZooAnimal::Frog => {
                write!(f, "images/animals/zoo/frog.png")
            }
            ZooAnimal::Goat => {
                write!(f, "images/animals/zoo/goat.png")
            }
            ZooAnimal::Goose => {
                write!(f, "images/animals/zoo/goose.png")
            }
            ZooAnimal::Gorilla => {
                write!(f, "images/animals/zoo/gorilla.png")
            }
            ZooAnimal::Monkey => {
                write!(f, "images/animals/zoo/monkey.png")
            }
            ZooAnimal::Moose => {
                write!(f, "images/animals/zoo/moose.png")
            }
            ZooAnimal::Pig => {
                write!(f, "images/animals/zoo/pig.png")
            }
            ZooAnimal::Porcupine => {
                write!(f, "images/animals/zoo/porcupine.png")
            }
            ZooAnimal::Sheep => {
                write!(f, "images/animals/zoo/sheep.png")
            }
            ZooAnimal::Skunk => {
                write!(f, "images/animals/zoo/skunk.png")
            }
            ZooAnimal::Toad => {
                write!(f, "images/animals/zoo/toad.png")
            }
            ZooAnimal::Turtle => {
                write!(f, "images/animals/zoo/turtle.png")
            }
            ZooAnimal::Wolf => {
                write!(f, "images/animals/zoo/wolf.png")
            }
            ZooAnimal::None => {
                write!(f, "") // No asset to load
            }
        }
    }
}
