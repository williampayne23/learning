use bevy::prelude::*;
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Clicker".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup);
    }
}

#[derive(Debug, Event)]
pub struct GameStartEvent;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d,));
    commands.trigger(GameStartEvent);
}
