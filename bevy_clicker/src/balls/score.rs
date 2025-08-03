use bevy::prelude::*;
use bevy_butler::*;

use crate::balls::BallsPlugin;

// Component to track score added this frame
// At the end of the frame we will trigger a score event with the total score

#[derive(Component, Default)]
pub struct Score {
    pub sum_score: f32,
    pub multiplier: f32,
}

#[derive(Event)]
pub struct AddScoreEvent(pub f32);

impl From<f32> for AddScoreEvent {
    fn from(val: f32) -> AddScoreEvent {
        AddScoreEvent(val)
    }
}

#[derive(Event)]
pub struct MultiplyScoreEvent(pub f32);

impl From<f32> for MultiplyScoreEvent {
    fn from(val: f32) -> MultiplyScoreEvent {
        MultiplyScoreEvent(val)
    }
}

// Commits score at the end of all score-related systems
#[derive(Event)]
pub struct CommitScoreEvent {
    pub amount: u32,
}

#[add_observer(plugin = BallsPlugin)]
fn add_score(
    trigger: Trigger<AddScoreEvent>,
    mut commands: Commands,
    mut score_query: Query<&mut Score>,
) {
    // If the entity does not have a Score component, we add one with the given score
    // If it does, we add the score to the existing Score component
    if let Ok(mut score) = score_query.get_mut(trigger.target()) {
        score.sum_score += trigger.0;
    } else {
        commands.entity(trigger.target()).insert(Score {
            sum_score: trigger.0,
            multiplier: 1.0,
        });
    }
}

#[add_observer(plugin = BallsPlugin)]
fn multiply_score(
    trigger: Trigger<MultiplyScoreEvent>,
    mut commands: Commands,
    mut score_query: Query<&mut Score>,
) {
    // If the entity does not have a Score component, we add one with the given multiplier
    // If it does, we multiply the existing Score component's multiplier
    if let Ok(mut score) = score_query.get_mut(trigger.target()) {
        score.multiplier *= trigger.0;
    } else {
        commands.entity(trigger.target()).insert(Score {
            sum_score: 0.0,
            multiplier: trigger.0,
        });
    }
}

#[add_system(schedule = PostUpdate, plugin = BallsPlugin)]
fn commit_scores(mut commands: Commands, score_query: Query<(Entity, &Score)>) {
    if score_query.is_empty() {
        return;
    }
    let total_score: f32 = score_query
        .iter()
        .map(|(_entity, score)| score.sum_score * score.multiplier)
        .sum();
    commands.trigger(CommitScoreEvent {
        amount: total_score as u32,
    });

    for (entity, _) in score_query.iter() {
        commands.entity(entity).remove::<Score>();
    }
}
