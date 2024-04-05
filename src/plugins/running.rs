use bevy::prelude::{App, Plugin};

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_systems(Update, spawn_alien)
        //     .add_systems(Update, spawn_alien_laser)
        //     .add_systems(Update, alien_laser_movement)
        //     .add_systems(Update, alien_laser_lifetime)
        //     .add_systems(Update, alien_movement)
        //     .add_systems(Update, spawn_player_laser)
        //     .add_systems(Update, player_laser_movement)
        //     .add_systems(Update, player_laser_lifetime)
        //     .add_systems(Update, spawn_player_torpedo)
        //     .add_systems(Update, player_torpedo_movement)
        //     .add_systems(Update, player_torpedo_lifetime)
        //     .add_systems(Update, player_movement)
        //     .add_systems(Update, player_weapon_select);
    }
}
