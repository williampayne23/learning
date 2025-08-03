use crate::balls::{BallHitWallEvent, BallsPlugin, WallDirection, score::MultiplyScoreEvent};
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct PreferredWall {
    pub wall_direction: WallDirection,
    pub score_multiplier: f32,
}

#[add_observer(plugin = BallsPlugin)]
fn preferred_wall_score_multiplier(
    trigger: Trigger<BallHitWallEvent>,
    mut commands: Commands,
    preferred_wall_query: Query<&PreferredWall>,
) {
    if let Some(preferred_wall) = preferred_wall_query.get(trigger.target()).ok() {
        if preferred_wall.wall_direction == trigger.wall_direction {
            commands.trigger_targets(
                MultiplyScoreEvent(preferred_wall.score_multiplier),
                trigger.target(),
            )
        }
    }
}
