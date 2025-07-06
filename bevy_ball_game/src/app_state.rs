use bevy::prelude::*;
pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(Update, (open_menu, start_game));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

pub fn open_menu(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && app_state.get() != &AppState::MainMenu {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
        println!("Opened Main menu");
    }
}

pub fn start_game(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::G) && app_state.get() != &AppState::Game {
        commands.insert_resource(NextState(Some(AppState::Game)));
        println!("Started Game");
    }
}
