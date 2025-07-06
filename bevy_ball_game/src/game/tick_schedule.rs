use bevy::prelude::*;

use crate::app_state::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MovementSet {
    Direction,
    Movement,
    CollisionChecking,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct Simulation;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct GameStart;
pub struct TickSchedulePlugin;

impl Plugin for TickSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(
            Update,
            Simulation
                .run_if(in_state(SimulationState::Running).and_then(in_state(AppState::Game))),
        )
        .configure_set(
            Update,
            MovementSet::Direction
                .before(MovementSet::Movement)
                .in_set(Simulation),
        )
        .configure_set(
            Update,
            MovementSet::Movement
                .before(MovementSet::CollisionChecking)
                .in_set(Simulation),
        );
    }
}
