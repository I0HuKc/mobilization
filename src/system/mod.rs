pub mod camera;
pub mod window;

use bevy::ecs::schedule::SystemLabel;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub(super) enum Systems {
    Camera,
    Window,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    InGame,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::MainMenu
    }
}
