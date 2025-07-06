use bevy::prelude::*;

mod balls;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(balls::BallsPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Event)]
pub struct GameStartEvent;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.send_event(GameStartEvent);
}
