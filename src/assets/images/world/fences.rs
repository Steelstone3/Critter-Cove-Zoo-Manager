use std::fmt::Display;

#[derive(PartialEq, Clone, Copy)]
pub enum WorldFence {
    Fence1,
    Fence2,
    Fence3,
    Fence4,
    None,
}

impl Display for WorldFence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldFence::Fence1 => write!(f, "images/world/fences/fence_1.png"),
            WorldFence::Fence2 => write!(f, "images/world/fences/fence_2.png"),
            WorldFence::Fence3 => write!(f, "images/world/fences/fence_3.png"),
            WorldFence::Fence4 => write!(f, "images/world/fences/fence_4.png"),
            WorldFence::None => write!(f, ""),
        }
    }
}
