
use rand_derive2::RandGen;
use std::fmt::Display;

#[derive(RandGen)]
pub enum GameAnimal {
    AnimatedBoar,
    Boar,
}

impl Display for GameAnimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameAnimal::AnimatedBoar => {
                write!(f, "images/animals/zoo/boar.gif")
            }
            GameAnimal::Boar => {
                write!(f, "images/animals/zoo/boar.png")
            }
        }
    }
}
