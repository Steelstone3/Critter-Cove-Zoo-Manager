use std::fmt::Display;

pub enum SpawnMenuIcon {
    Animals,
    Fences,
    Terrains,
    Trees,
    Rocks,
    Paths,
}

impl Display for SpawnMenuIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpawnMenuIcon::Animals => {
                write!(f, "images/user_interface/main_menu/animals_icon.png")
            }
            SpawnMenuIcon::Fences => {
                write!(f, "images/user_interface/main_menu/fences_icon.png")
            }
            SpawnMenuIcon::Terrains => {
                write!(f, "images/user_interface/main_menu/terrains_icon.png")
            }
            SpawnMenuIcon::Trees => {
                write!(f, "images/user_interface/main_menu/trees_icon.png")
            }
            SpawnMenuIcon::Rocks => {
                write!(f, "images/user_interface/main_menu/rocks_icon.png")
            }
            SpawnMenuIcon::Paths => {
                write!(f, "images/user_interface/main_menu/paths_icon.png")
            }
        }
    }
}
