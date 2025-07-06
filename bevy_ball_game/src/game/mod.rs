use bevy::prelude::*;

use crate::app_state::AppState;

use self::{
    collide::CollidePlugin, enemy::EnemyPlugin, game_logic::GameLogicPlugin, player::PlayerPlugin,
    star::StarPlugin, tick_schedule::TickSchedulePlugin,
};
mod collide;
pub mod enemy;
pub mod game_logic;
mod player;
mod star;
mod tick_schedule;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_plugins(TickSchedulePlugin)
            .add_plugins((
                CollidePlugin,
                EnemyPlugin,
                GameLogicPlugin,
                PlayerPlugin,
                StarPlugin,
            ))
            .add_systems(Update, toggle_simulation)
            .add_systems(OnExit(AppState::Game), despawn_on_game_end);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        let new_state = match simulation_state.get() {
            SimulationState::Running => SimulationState::Paused,
            SimulationState::Paused => SimulationState::Running,
        };
        commands.insert_resource(NextState(Some(new_state)));
        println!("{:?}", new_state)
    }
}

#[derive(Component)]
pub struct DespawnOnEnd;

pub fn despawn_on_game_end(
    mut commands: Commands,
    despawn_entities: Query<Entity, With<DespawnOnEnd>>,
) {
    for e in despawn_entities.iter() {
        commands.entity(e).despawn();
    }
}
