use bevy::prelude::*;
use bevy_butler::*;

use crate::input::InputPlugin;

#[derive(Event)]
pub struct MouseClickEvent {
    pub pos: Vec2,
}

// Trigger `ExplodeMines` at the position of a given click
#[add_system(plugin = InputPlugin, schedule = Update)]
fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera;
    if let Some(pos) = windows
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.trigger(MouseClickEvent { pos });
        }
    }
}
