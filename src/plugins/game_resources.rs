use bevy::prelude::{App, Plugin};

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(WindowSize::default())
        //     .insert_resource(Points::default())
        //     .insert_resource(LaserAmmunition(10.0))
        //     .insert_resource(TorpedoAmmunition(3.0))
        //     .insert_resource(SelectedWeapon(1))
        //     .insert_resource(EnemyCount(Default::default()))
        //     .insert_resource(EnemyLimit(1));
    }
}
