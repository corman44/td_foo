use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenu {}

#[derive(Component)]
pub enum PauseMenuButtonAction {
    Settings,
    Quit,
}