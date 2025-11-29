use bevy::prelude::*;
use bevy_butler::*;
use components::button;
use haalka::prelude::*;

pub mod components;

/// Only sets up dependencies for UI functionality.
/// Actually spawning the UI elements is for other plugins to handle.
/// This library should just export builders
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HaalkaPlugin);
        //.add_systems(Startup, setup_ui);
    }
}

// fn setup_ui(world: &mut World) {
//     button::ui_root().spawn(world);
// }
