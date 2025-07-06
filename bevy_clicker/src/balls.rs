use bevy_butler::*;
use bevy::prelude::*;

use crate::GameStartEvent;

#[butler_plugin]
pub struct BallsPlugin;

#[derive(Component)]
pub struct Ball {
    pub radius: f32,
    pub velocity: Vec2,
    pub position: Vec2,
}

#[add_observer(plugin = BallsPlugin)]
pub fn ball_spawner(
    _trigger: Trigger<GameStartEvent>,
    mut commands: Commands,
) {
    println!("Spawning balls...");
}



#[derive(Debug, Event)]
pub struct CollideBallEvent();

// Spawn ball system

// Move ball system
// #[add_system(plugin = FoodPlugin, schedule = Update)]
// fn move_ball(
//     mut commands: Commands,
// ) {
//     println!("Moving balls...");
// }

