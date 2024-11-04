use crate::assets::images::user_interface::tree_icons::TreeIcon;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TreeSprite {
    Bush1,
    Bush2,
    TallGrass1,
    TallGrass2,
    TallGrass3,
    Tree1,
    Tree2,
    Tree3,
    Tree4,
    None,
}

impl Display for TreeSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeSprite::Bush1 => write!(f, "images/world/trees/bush_1.png"),
            TreeSprite::Bush2 => write!(f, "images/world/trees/bush_2.png"),
            TreeSprite::TallGrass1 => write!(f, "images/world/trees/tall_grass_1.png"),
            TreeSprite::TallGrass2 => write!(f, "images/world/trees/tall_grass_2.png"),
            TreeSprite::TallGrass3 => write!(f, "images/world/trees/tall_grass_3.png"),
            TreeSprite::Tree1 => write!(f, "images/world/trees/tree_1.png"),
            TreeSprite::Tree2 => write!(f, "images/world/trees/tree_2.png"),
            TreeSprite::Tree3 => write!(f, "images/world/trees/tree_3.png"),
            TreeSprite::Tree4 => write!(f, "images/world/trees/tree_4.png"),
            TreeSprite::None => write!(f, ""),
        }
    }
}

impl TreeSprite {
    pub fn convert_from(icon: TreeIcon) -> Self {
        match icon {
            TreeIcon::Bush1 => TreeSprite::Bush1,
            TreeIcon::Bush2 => TreeSprite::Bush2,
            TreeIcon::TallGrass1 => TreeSprite::TallGrass1,
            TreeIcon::TallGrass2 => TreeSprite::TallGrass2,
            TreeIcon::TallGrass3 => TreeSprite::TallGrass3,
            TreeIcon::Tree1 => TreeSprite::Tree1,
            TreeIcon::Tree2 => TreeSprite::Tree2,
            TreeIcon::Tree3 => TreeSprite::Tree3,
            TreeIcon::Tree4 => TreeSprite::Tree4,
        }
    }
}

#[cfg(test)]
mod tree_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TreeIcon::Bush1, TreeSprite::Bush1)]
    #[case(TreeIcon::Bush2, TreeSprite::Bush2)]
    #[case(TreeIcon::TallGrass1, TreeSprite::TallGrass1)]
    #[case(TreeIcon::TallGrass2, TreeSprite::TallGrass2)]
    #[case(TreeIcon::TallGrass3, TreeSprite::TallGrass3)]
    #[case(TreeIcon::Tree1, TreeSprite::Tree1)]
    #[case(TreeIcon::Tree2, TreeSprite::Tree2)]
    #[case(TreeIcon::Tree3, TreeSprite::Tree3)]
    #[case(TreeIcon::Tree4, TreeSprite::Tree4)]
    fn convert_from(#[case] icon: TreeIcon, #[case] expected_sprite: TreeSprite) {
        // when
        let actual_sprite = TreeSprite::convert_from(icon);

        // then
        assert_eq!(expected_sprite, actual_sprite);
    }
}
