use bevy::prelude::*;

use self::{main_menu::MainMenuPlugin, pause_menu::PauseMenuPlugin};

pub mod style;
pub mod main_menu;
pub mod pause_menu;
pub mod interactions;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_plugins((MainMenuPlugin, PauseMenuPlugin));
    }
}