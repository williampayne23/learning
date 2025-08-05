use bevy::prelude::*;
use bevy_butler::*;
use components::button;
use haalka::prelude::*;

pub mod components;

#[butler_plugin]
pub struct UIPlugin;

#[add_system(plugin = UIPlugin, schedule = Startup)]
fn setup_ui(world: &mut World) {
    button::ui_root().spawn(world);
}
