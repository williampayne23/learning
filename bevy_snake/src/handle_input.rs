use bevy::prelude::*;
use bevy_butler::*;

use super::direction::Direction;

#[butler_plugin]
pub struct HandleInputPlugin;

#[derive(Resource)]
#[insert_resource(plugin = HandleInputPlugin, init = LastDirectionPressed(Direction::Right))]
pub struct LastDirectionPressed(pub Direction);

#[add_system(plugin = HandleInputPlugin, schedule = Update)]
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut last_direction_pressed: ResMut<LastDirectionPressed>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        last_direction_pressed.0 = Direction::Up;
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        last_direction_pressed.0 = Direction::Down;
    } else if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        last_direction_pressed.0 = Direction::Left;
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        last_direction_pressed.0 = Direction::Right;
    }
}
