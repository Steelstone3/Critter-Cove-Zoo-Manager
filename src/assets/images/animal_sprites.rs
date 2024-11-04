use std::fmt::Display;

use super::user_interface::zoo_animal_icons::AnimalIcon;

#[derive(Debug, Clone, Copy, PartialEq)]
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

impl AnimalSprite {
    pub fn convert_from(icon: AnimalIcon) -> Self {
        match icon {
            AnimalIcon::Boar => AnimalSprite::Boar,
            AnimalIcon::Chicken => AnimalSprite::Chicken,
            AnimalIcon::Cow => AnimalSprite::Cow,
            AnimalIcon::Crab => AnimalSprite::Crab,
            AnimalIcon::Dog => AnimalSprite::Dog,
            AnimalIcon::Fox => AnimalSprite::Fox,
            AnimalIcon::Frog => AnimalSprite::Frog,
            AnimalIcon::Goat => AnimalSprite::Goat,
            AnimalIcon::Goose => AnimalSprite::Goose,
            AnimalIcon::Gorilla => AnimalSprite::Gorilla,
            AnimalIcon::Monkey => AnimalSprite::Monkey,
            AnimalIcon::Moose => AnimalSprite::Moose,
            AnimalIcon::Pig => AnimalSprite::Pig,
            AnimalIcon::Porcupine => AnimalSprite::Porcupine,
            AnimalIcon::Sheep => AnimalSprite::Sheep,
            AnimalIcon::Skunk => AnimalSprite::Skunk,
            AnimalIcon::Toad => AnimalSprite::Toad,
            AnimalIcon::Turtle => AnimalSprite::Turtle,
            AnimalIcon::Wolf => AnimalSprite::Wolf,
        }
    }
}

#[cfg(test)]
mod animal_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(AnimalIcon::Boar, AnimalSprite::Boar)]
    #[case(AnimalIcon::Chicken, AnimalSprite::Chicken)]
    #[case(AnimalIcon::Cow, AnimalSprite::Cow)]
    #[case(AnimalIcon::Crab, AnimalSprite::Crab)]
    #[case(AnimalIcon::Dog, AnimalSprite::Dog)]
    #[case(AnimalIcon::Fox, AnimalSprite::Fox)]
    #[case(AnimalIcon::Frog, AnimalSprite::Frog)]
    #[case(AnimalIcon::Goat, AnimalSprite::Goat)]
    #[case(AnimalIcon::Goose, AnimalSprite::Goose)]
    #[case(AnimalIcon::Monkey, AnimalSprite::Monkey)]
    #[case(AnimalIcon::Pig, AnimalSprite::Pig)]
    #[case(AnimalIcon::Porcupine, AnimalSprite::Porcupine)]
    #[case(AnimalIcon::Sheep, AnimalSprite::Sheep)]
    #[case(AnimalIcon::Skunk, AnimalSprite::Skunk)]
    #[case(AnimalIcon::Toad, AnimalSprite::Toad)]
    #[case(AnimalIcon::Turtle, AnimalSprite::Turtle)]
    #[case(AnimalIcon::Wolf, AnimalSprite::Wolf)]
    #[case(AnimalIcon::Gorilla, AnimalSprite::Gorilla)]
    #[case(AnimalIcon::Moose, AnimalSprite::Moose)]
    fn convert_from(#[case] icon: AnimalIcon, #[case] expected_sprite: AnimalSprite) {
        // when
        let actual_sprite = AnimalSprite::convert_from(icon);

        // then
        assert_eq!(expected_sprite, actual_sprite);
    }
}
