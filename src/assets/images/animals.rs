use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum AnimalSprite {
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

impl Display for AnimalSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // 16 bit
            // Zoo
            AnimalSprite::Boar => {
                write!(f, "images/animals/zoo/boar.png")
            }
            AnimalSprite::Chicken => {
                write!(f, "images/animals/zoo/chicken.png")
            }
            AnimalSprite::Cow => {
                write!(f, "images/animals/zoo/cow.png")
            }
            AnimalSprite::Crab => {
                write!(f, "images/animals/zoo/crab.png")
            }
            AnimalSprite::Dog => {
                write!(f, "images/animals/zoo/dog.png")
            }
            AnimalSprite::Fox => {
                write!(f, "images/animals/zoo/fox.png")
            }
            AnimalSprite::Frog => {
                write!(f, "images/animals/zoo/frog.png")
            }
            AnimalSprite::Goat => {
                write!(f, "images/animals/zoo/goat.png")
            }
            AnimalSprite::Goose => {
                write!(f, "images/animals/zoo/goose.png")
            }
            AnimalSprite::Monkey => {
                write!(f, "images/animals/zoo/monkey.png")
            }
            AnimalSprite::Pig => {
                write!(f, "images/animals/zoo/pig.png")
            }
            AnimalSprite::Porcupine => {
                write!(f, "images/animals/zoo/porcupine.png")
            }
            AnimalSprite::Sheep => {
                write!(f, "images/animals/zoo/sheep.png")
            }
            AnimalSprite::Skunk => {
                write!(f, "images/animals/zoo/skunk.png")
            }
            AnimalSprite::Toad => {
                write!(f, "images/animals/zoo/toad.png")
            }
            AnimalSprite::Turtle => {
                write!(f, "images/animals/zoo/turtle.png")
            }
            AnimalSprite::Wolf => {
                write!(f, "images/animals/zoo/wolf.png")
            }
            // Chungus 32 bit
            // Zoo
            AnimalSprite::Gorilla => {
                write!(f, "images/animals/zoo/gorilla.png")
            }
            AnimalSprite::Moose => {
                write!(f, "images/animals/zoo/moose.png")
            }
            // Monsters
            AnimalSprite::RearingNightmare => {
                write!(f, "images/animals/monsters/rearing_nightmare.png")
            }
            AnimalSprite::StormGiant => {
                write!(f, "images/animals/monsters/storm_giant.png")
            }
            // No asset to load
            AnimalSprite::None => {
                write!(f, "")
            }
        }
    }
}
