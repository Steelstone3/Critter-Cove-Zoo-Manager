use crate::assets::images::user_interface::main_menu::SpawnMenuIcon;

#[derive(Debug, PartialEq)]
pub enum SpawnMenu {
    None,
    Animals,
    Fences,
    Terrains,
    Trees,
    Rocks,
    // Shelters,
    Paths,
}

impl SpawnMenu {
    pub fn convert_from(icon: SpawnMenuIcon) -> Self {
        match icon {
            SpawnMenuIcon::Animals => SpawnMenu::Animals,
            SpawnMenuIcon::Fences => SpawnMenu::Fences,
            SpawnMenuIcon::Terrains => SpawnMenu::Terrains,
            SpawnMenuIcon::Trees => SpawnMenu::Trees,
            SpawnMenuIcon::Rocks => SpawnMenu::Rocks,
            SpawnMenuIcon::Paths => SpawnMenu::Paths,
        }
    }
}

#[cfg(test)]
mod spawn_menu_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(SpawnMenuIcon::Animals, SpawnMenu::Animals)]
    #[case(SpawnMenuIcon::Fences, SpawnMenu::Fences)]
    #[case(SpawnMenuIcon::Terrains, SpawnMenu::Terrains)]
    #[case(SpawnMenuIcon::Trees, SpawnMenu::Trees)]
    #[case(SpawnMenuIcon::Rocks, SpawnMenu::Rocks)]
    #[case(SpawnMenuIcon::Paths, SpawnMenu::Paths)]
    fn convert_from(#[case] icon: SpawnMenuIcon, #[case] expected_sprite: SpawnMenu) {
        // when
        let actual_sprite = SpawnMenu::convert_from(icon);

        // then
        assert_eq!(expected_sprite, actual_sprite);
    }
}
