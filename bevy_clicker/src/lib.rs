use bevy::prelude::*;

mod balls;
mod input;
mod player;
mod setup;
mod shop;
mod ui;
mod utilities;
mod world_to_screen;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            balls::BallsPlugin,
            input::InputPlugin,
            player::PlayerPlugin,
            setup::SetupPlugin,
            shop::ShopPlugin,
            ui::UIPlugin,
            utilities::UtilitiesPlugin,
            world_to_screen::WorldToScreenPLugin {
                width: 100.,
                height: 100.,
            },
        ));
    }
}
