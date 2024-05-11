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

#[derive(Component)]
pub struct AttackButton;

#[derive(Component)]
pub struct DefendButton;

#[derive(Component)]
pub struct MultiplayerButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct QuitButton;