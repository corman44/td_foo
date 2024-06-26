mod game;
mod menus;
mod debugging;

use bevy::{app::AppExit, prelude::*, window::{PrimaryWindow, WindowResolution}};
use bevy_kira_audio::prelude::*;

use debugging::DebuggingPlugin;
use game::{GamePlugin, GameState};
use menus::MenusPlugin;

pub struct AppPlugin;

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum AppState {
    Game,
    GameOver,
    #[default]
    MainMenu,
    PauseMenu,
}

#[derive(Debug, Event)]
pub struct GameOver {
    pub score: u32
}

#[derive(Component)]
struct MyCameraMarker;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_state::<AppState>()
            .add_event::<GameOver>()
            .add_plugins((
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resolution: WindowResolution::new(1024., 1024.).with_scale_factor_override(1.0),
                            ..default()
                        }),
                        ..default()
                    }),
                AudioPlugin,
                GamePlugin,
                DebuggingPlugin,
                MenusPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                handle_game_over,
                transition_to_main_menu_state));
    }
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>> 
) {
    for event in game_over_event_reader.read() {
        info!("Final Score: {}", event.score.to_string());
        next_app_state.set(AppState::GameOver);
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        MyCameraMarker,
    ));
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<GameState>>,
) {

    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.get() != &AppState::MainMenu {
            info!("AppState: MainMenu");
            next_app_state.set(AppState::MainMenu);
            next_sim_state.set(GameState::Paused);
        }
    }
}

