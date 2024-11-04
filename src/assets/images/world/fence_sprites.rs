use std::fmt::Display;

use crate::assets::images::user_interface::fence_icons::FenceIcon;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FenceSprite {
    Fence1,
    Fence2,
    Fence3,
    Fence4,
    None,
}

impl Display for FenceSprite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenceSprite::Fence1 => write!(f, "images/world/fences/fence_1.png"),
            FenceSprite::Fence2 => write!(f, "images/world/fences/fence_2.png"),
            FenceSprite::Fence3 => write!(f, "images/world/fences/fence_3.png"),
            FenceSprite::Fence4 => write!(f, "images/world/fences/fence_4.png"),
            FenceSprite::None => write!(f, ""),
        }
    }
}

impl FenceSprite {
    pub fn convert_from(icon: FenceIcon) -> Self {
        match icon {
            FenceIcon::Fence1 => FenceSprite::Fence1,
            FenceIcon::Fence2 => FenceSprite::Fence2,
            FenceIcon::Fence3 => FenceSprite::Fence3,
            FenceIcon::Fence4 => FenceSprite::Fence4,
        }
    }
}

#[cfg(test)]
mod fence_sprite_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(FenceIcon::Fence1, FenceSprite::Fence1)]
    #[case(FenceIcon::Fence2, FenceSprite::Fence2)]
    #[case(FenceIcon::Fence3, FenceSprite::Fence3)]
    #[case(FenceIcon::Fence4, FenceSprite::Fence4)]
    fn convert_from(#[case] icon: FenceIcon, #[case] expected_sprite: FenceSprite) {
        // when
        let actual_sprite = FenceSprite::convert_from(icon);

        // then
        assert_eq!(expected_sprite, actual_sprite);
    }
}
