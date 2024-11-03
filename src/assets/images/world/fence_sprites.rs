use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
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
