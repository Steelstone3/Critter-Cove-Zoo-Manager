use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum ZooAnimal {
    // 16 bit
    // Zoo
    Boar,
    Chicken,
    Cow,
    Crab,
    Dog,
    Fox,
    Frog,
    Goat,
    Goose,
    Monkey,
    Pig,
    Porcupine,
    Sheep,
    Skunk,
    Toad,
    Turtle,
    Wolf,
    // Chungus 32 bit
    // Zoo
    Gorilla,
    Moose,
    // Monsters
    RearingNightmare,
    StormGiant,
    // Nothing selected
    None,
}

impl Display for ZooAnimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // 16 bit
            // Zoo
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
            ZooAnimal::Monkey => {
                write!(f, "images/animals/zoo/monkey.png")
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
            // Chungus 32 bit
            // Zoo
            ZooAnimal::Gorilla => {
                write!(f, "images/animals/zoo/gorilla.png")
            }
            ZooAnimal::Moose => {
                write!(f, "images/animals/zoo/moose.png")
            }
            // Monsters
            ZooAnimal::RearingNightmare => {
                write!(f, "images/animals/monsters/rearing_nightmare.png")
            }
            ZooAnimal::StormGiant => {
                write!(f, "images/animals/monsters/storm_giant.png")
            }
            // No asset to load
            ZooAnimal::None => {
                write!(f, "")
            }
        }
    }
}
