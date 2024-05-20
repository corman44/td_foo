use bevy::prelude::*;

use self::system::{check_mouse_click, track_click_and_drag};

pub mod system;
pub mod components;

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<MouseSelectionState>()
            // .add_systems(Update, track_click_and_drag)
            .add_systems(Update, check_mouse_click
                .run_if(in_state(MouseSelectionState::NewDefenderSelected)))
            ;
    }
}

#[derive(Default, Debug, States, Hash, PartialEq, Eq, Clone)]
pub enum MouseSelectionState {
    AttackerSelected,
    DefenderSelected,
    #[default]
    Idle,
    NewDefenderSelected,
}

#[derive(Default, Debug, Resource)]
pub struct SelectedEntityID(i32);
