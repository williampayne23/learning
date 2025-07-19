use bevy::{prelude::*, window::PrimaryWindow};

mod balls;
mod world_to_screen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Clicker".to_string(),
                resolution: (600.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(balls::BallsPlugin)
        .add_plugins(world_to_screen::WorldToScreenPLugin {
            width: 100.,
            height: 100.,
        })
        .add_systems(Startup, setup)
        .run();
}

#[derive(Debug, Event)]
pub struct GameStartEvent;

fn setup(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single().expect("No primary window found");
    println!("Window size: {:?}", window.resolution);
    commands.spawn((Camera2d,));
    commands.trigger(GameStartEvent);
}
