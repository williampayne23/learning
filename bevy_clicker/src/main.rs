use bevy::prelude::*;

mod balls;
pub mod input;
mod player;
mod shop;
mod world_to_screen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Clicker".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(balls::BallsPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(shop::ShopPlugin)
        .add_plugins(world_to_screen::WorldToScreenPLugin {
            width: 100.,
            height: 100.,
        })
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Event)]
pub struct GameStartEvent;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d,));
    commands.trigger(GameStartEvent);
}
