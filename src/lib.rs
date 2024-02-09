mod game;
mod main_menu;

use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};
use bevy_kira_audio::prelude::*;
use game::{GamePlugin, GameState};

pub struct AppPlugin;

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[derive(Event)]
pub struct GameOver {
    pub score: u32
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_state::<AppState>()
            .add_event::<GameOver>()
            .add_plugins((DefaultPlugins, AudioPlugin, GamePlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (handle_game_over, exit_game, transition_to_game_state, transition_to_main_menu_state));
    }

    fn ready(&self, _app: &App) -> bool {
        true
    }

    fn finish(&self, _app: &mut App) {
        // do nothing
    }

    fn cleanup(&self, _app: &mut App) {
        // do nothing
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    fn is_unique(&self) -> bool {
        true
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut game_over_event_writer: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>
) {
    for event in game_over_event_writer.read() {
        println!("Final Score: {}", event.score.to_string());
        next_app_state.set(AppState::GameOver);
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.get() != &AppState::Game {
            println!("AppState: Game");
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<GameState>>,
) {

    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.get() != &AppState::MainMenu {
            println!("AppState: MainMenu");
            next_app_state.set(AppState::MainMenu);
            next_sim_state.set(GameState::Paused);
        }
    }
}
