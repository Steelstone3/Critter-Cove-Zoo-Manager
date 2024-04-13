use bevy::{
    ecs::{query::Changed, system::Query},
    ui::Interaction,
};

use crate::components::user_interface::SelectAnimalButton;

pub fn select_animal_button(
    mut select_animal_button_queries: Query<
        (&SelectAnimalButton, &Interaction),
        Changed<Interaction>,
    >,
    
) {
    let Ok(select_animal_button_query) = select_animal_button_queries.get_single() else {
        return;
    };

    match *select_animal_button_query.1 {
        Interaction::Pressed => todo!(),
        Interaction::Hovered => todo!(),
        Interaction::None => todo!(),
    }
}
