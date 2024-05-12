use bevy::prelude::*;

use crate::game::attacker::AttackerSpawnState;

use super::{components::StartRoundButton, style::{HOVER_START_BUTTON_COLOR, START_BUTTON_COLOR}};


pub fn interact_start_round_button(
    mut next_attacker_spawn_state: ResMut<NextState<AttackerSpawnState>>,
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction> ,With<StartRoundButton>)>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                next_attacker_spawn_state.set(AttackerSpawnState::Spawning);
                *background_color = HOVER_START_BUTTON_COLOR.into();
            },
            Interaction::Hovered => {
                *background_color = HOVER_START_BUTTON_COLOR.into();
            },
            Interaction::None => {
                *background_color = START_BUTTON_COLOR.into();
            },
        }
    }
}
