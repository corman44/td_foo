use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub enum MenuButtonAction {
    Attack,
    Defend,
    Multiplayer,
    Settings,
    Quit,
}
