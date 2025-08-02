use crate::balls::{Ball, BallHitWallEvent, BallsPlugin, WallDirection};
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct BaseScore {
    pub score: u32,
}

impl Default for BaseScore {
    fn default() -> Self {
        Self { score: 1 }
    }
}

#[add_system(plugin = BallsPlugin, schedule = Update)]
fn add_base_score_on_ball_hit_wall(
    mut commands: Commands,
    mut ball_query: Query<&mut Ball>,
    trigger: Trigger<BallHitWallEvent>,
) {
    if let Ok(mut ball) = ball_query.get_mut(trigger.target()) {
        // Add base score to the ball
        ball.base_score = Some(BaseScore::default());
    }

    // If the wall direction is horizontal, we can add a base score
    if let Some(wall_direction) = trigger.event().wall_direction {
        if wall_direction == WallDirection::Horizontal {
            commands
                .entity(trigger.target())
                .insert(BaseScore::default());
        }
    }
}
