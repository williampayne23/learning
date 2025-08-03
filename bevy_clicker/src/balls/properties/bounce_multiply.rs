use crate::balls::{BallHitWallEvent, BallsPlugin, score::MultiplyScoreEvent};
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct MultOnWallBounce {
    pub score: f32,
}

impl Default for MultOnWallBounce {
    fn default() -> Self {
        Self { score: 1. }
    }
}

#[add_observer(plugin = BallsPlugin)]
fn score_on_bounce(
    trigger: Trigger<BallHitWallEvent>,
    mut commands: Commands,
    score_query: Query<&MultOnWallBounce>,
) {
    if let Some(bounce_score) = score_query.get(trigger.target()).ok() {
        commands.trigger_targets(MultiplyScoreEvent(bounce_score.score), trigger.target())
    }
}
