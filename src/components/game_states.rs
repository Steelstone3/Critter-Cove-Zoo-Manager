use bevy::ecs::schedule::States;

#[allow(dead_code)]
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    #[default] // TODO move this to main menu
    InGame,
    PauseMenu,
}
