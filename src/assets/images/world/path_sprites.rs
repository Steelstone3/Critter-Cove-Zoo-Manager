use std::fmt::Display;

use crate::assets::images::user_interface::path_icons::PathIcon;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PathSprite {
    Path1,
    Path2,
    Path3,
    Path4,
    Path5,
    Path6,
    Path7,
    Path8,
    Path9,
    Path10,
    Path11,
    Path12,
    Path13,
    Path14,
    Path15,
    Path16,
    Path17,
    Path18,
    Path19,
    Path20,
    Path21,
    Path22,
    Path23,
    None,
}

impl Display for PathSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathSprite::Path1 => write!(f, "images/world/paths/path_1.png"),
            PathSprite::Path2 => write!(f, "images/world/paths/path_2.png"),
            PathSprite::Path3 => write!(f, "images/world/paths/path_3.png"),
            PathSprite::Path4 => write!(f, "images/world/paths/path_4.png"),
            PathSprite::Path5 => write!(f, "images/world/paths/path_5.png"),
            PathSprite::Path6 => write!(f, "images/world/paths/path_6.png"),
            PathSprite::Path7 => write!(f, "images/world/paths/path_7.png"),
            PathSprite::Path8 => write!(f, "images/world/paths/path_8.png"),
            PathSprite::Path9 => write!(f, "images/world/paths/path_9.png"),
            PathSprite::Path10 => write!(f, "images/world/paths/path_10.png"),
            PathSprite::Path11 => write!(f, "images/world/paths/path_11.png"),
            PathSprite::Path12 => write!(f, "images/world/paths/path_12.png"),
            PathSprite::Path13 => write!(f, "images/world/paths/path_13.png"),
            PathSprite::Path14 => write!(f, "images/world/paths/path_14.png"),
            PathSprite::Path15 => write!(f, "images/world/paths/path_15.png"),
            PathSprite::Path16 => write!(f, "images/world/paths/path_16.png"),
            PathSprite::Path17 => write!(f, "images/world/paths/path_17.png"),
            PathSprite::Path18 => write!(f, "images/world/paths/path_18.png"),
            PathSprite::Path19 => write!(f, "images/world/paths/path_19.png"),
            PathSprite::Path20 => write!(f, "images/world/paths/path_20.png"),
            PathSprite::Path21 => write!(f, "images/world/paths/path_21.png"),
            PathSprite::Path22 => write!(f, "images/world/paths/path_22.png"),
            PathSprite::Path23 => write!(f, "images/world/paths/path_23.png"),
            PathSprite::None => write!(f, ""),
        }
    }
}

impl PathSprite {
    pub fn convert_from(icon: PathIcon) -> Self {
        match icon {
            PathIcon::Path1 => PathSprite::Path1,
            PathIcon::Path2 => PathSprite::Path2,
            PathIcon::Path3 => PathSprite::Path3,
            PathIcon::Path4 => PathSprite::Path4,
            PathIcon::Path5 => PathSprite::Path5,
            PathIcon::Path6 => PathSprite::Path6,
            PathIcon::Path7 => PathSprite::Path7,
            PathIcon::Path8 => PathSprite::Path8,
            PathIcon::Path9 => PathSprite::Path9,
            PathIcon::Path10 => PathSprite::Path10,
            PathIcon::Path11 => PathSprite::Path11,
            PathIcon::Path12 => PathSprite::Path12,
            PathIcon::Path13 => PathSprite::Path13,
            PathIcon::Path14 => PathSprite::Path14,
            PathIcon::Path15 => PathSprite::Path15,
            PathIcon::Path16 => PathSprite::Path16,
            PathIcon::Path17 => PathSprite::Path17,
            PathIcon::Path18 => PathSprite::Path18,
            PathIcon::Path19 => PathSprite::Path19,
            PathIcon::Path20 => PathSprite::Path20,
            PathIcon::Path21 => PathSprite::Path21,
            PathIcon::Path22 => PathSprite::Path22,
            PathIcon::Path23 => PathSprite::Path23,
        }
    }
}

#[cfg(test)]
mod fence_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(PathIcon::Path1, PathSprite::Path1)]
    #[case(PathIcon::Path2, PathSprite::Path2)]
    #[case(PathIcon::Path3, PathSprite::Path3)]
    #[case(PathIcon::Path4, PathSprite::Path4)]
    #[case(PathIcon::Path5, PathSprite::Path5)]
    #[case(PathIcon::Path6, PathSprite::Path6)]
    #[case(PathIcon::Path7, PathSprite::Path7)]
    #[case(PathIcon::Path8, PathSprite::Path8)]
    #[case(PathIcon::Path9, PathSprite::Path9)]
    #[case(PathIcon::Path10, PathSprite::Path10)]
    #[case(PathIcon::Path11, PathSprite::Path11)]
    #[case(PathIcon::Path12, PathSprite::Path12)]
    #[case(PathIcon::Path13, PathSprite::Path13)]
    #[case(PathIcon::Path14, PathSprite::Path14)]
    #[case(PathIcon::Path15, PathSprite::Path15)]
    #[case(PathIcon::Path16, PathSprite::Path16)]
    #[case(PathIcon::Path17, PathSprite::Path17)]
    #[case(PathIcon::Path18, PathSprite::Path18)]
    #[case(PathIcon::Path19, PathSprite::Path19)]
    #[case(PathIcon::Path20, PathSprite::Path20)]
    #[case(PathIcon::Path21, PathSprite::Path21)]
    #[case(PathIcon::Path22, PathSprite::Path22)]
    #[case(PathIcon::Path23, PathSprite::Path23)]
    fn convert_from(#[case] icon: PathIcon, #[case] expected_sprite: PathSprite) {
        // when
        let actual_sprite = PathSprite::convert_from(icon);

        // then
        assert_eq!(expected_sprite, actual_sprite);
    }
}
