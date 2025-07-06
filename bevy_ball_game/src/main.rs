use app_state::AppStatePlugin;
use bevy::prelude::*;
use game::GamePlugin;
use menus::MenusPlugin;
// use bevy::ui::debug;

mod app_state;
mod game;
mod menus;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((GamePlugin, AppStatePlugin, MenusPlugin))
        .run();
}
