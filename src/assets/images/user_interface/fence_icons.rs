use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum FenceIcon {
    Fence1,
    Fence2,
    Fence3,
    Fence4,
}

impl Display for FenceIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenceIcon::Fence1 => {
                write!(f, "images/user_interface/fence_menu/fence_1_icon.png")
            }
            FenceIcon::Fence2 => {
                write!(f, "images/user_interface/fence_menu/fence_2_icon.png")
            }
            FenceIcon::Fence3 => {
                write!(f, "images/user_interface/fence_menu/fence_3_icon.png")
            }
            FenceIcon::Fence4 => {
                write!(f, "images/user_interface/fence_menu/fence_4_icon.png")
            }
        }
    }
}
