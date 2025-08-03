use crate::balls::{BallHitWallEvent, BallsPlugin, score::AddScoreEvent};
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct ScoreOnWallBounce {
    pub score: f32,
}

impl Default for ScoreOnWallBounce {
    fn default() -> Self {
        Self { score: 1. }
    }
}

#[add_observer(plugin = BallsPlugin)]
fn score_on_bounce(
    trigger: Trigger<BallHitWallEvent>,
    mut commands: Commands,
    score_query: Query<&ScoreOnWallBounce>,
) {
    if let Some(bounce_score) = score_query.get(trigger.target()).ok() {
        commands.trigger_targets(AddScoreEvent(bounce_score.score), trigger.target())
    }
}
