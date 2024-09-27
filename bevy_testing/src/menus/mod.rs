mod common;
mod game_over;
mod hud;
mod main_menu;

use bevy::prelude::*;

use crate::app_state::AppState;

use self::{
    common::{animate_buttons, interact_with_play_button, interact_with_quit_button},
    game_over::GameOverMenuPlugin,
    hud::HudPlugin,
    main_menu::MainMenuPlugin,
};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MainMenuPlugin, GameOverMenuPlugin, HudPlugin))
            .add_systems(Update, animate_buttons)
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(not(in_state(AppState::Game))),
            );
    }
}
