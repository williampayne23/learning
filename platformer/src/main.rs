use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use platformer::camera::PlayerCameraPlugin;
use platformer::follow::FollowTargetPlugin;
use platformer::physics::PhysicsPlugin;
use platformer::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Platformer".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        )
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PlayerCameraPlugin)
        .add_plugins(FollowTargetPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Update, ticks)
        .run();
}

fn ticks(mut gizmos: Gizmos) {
    //Sprites to make it clear we're moving
    for i in 0..20 {
        gizmos.line(
            Vec3::new((i as f32) * 100. - 1000., -10., 0.),
            Vec3::new((i as f32) * 100. - 1000., 10., 0.),
            Color::RED,
        )
    }
}
