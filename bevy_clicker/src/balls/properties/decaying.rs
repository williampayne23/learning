use crate::balls::{Ball, BallHitWallEvent, BallsPlugin};
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct DecayRate {
    pub decay_rate: f32,
}

#[add_observer(plugin = BallsPlugin)]
fn shrink_on_bounce(
    trigger: Trigger<BallHitWallEvent>,
    mut commands: Commands,
    mut ball_query: Query<(&mut Ball, &DecayRate)>,
) {
    if let Some((mut ball, decay_rate)) = ball_query.get_mut(trigger.target()).ok() {
        ball.diameter -= decay_rate.decay_rate;
        // Shrink the ball by 10% on each bounce
        if ball.diameter < 1.0 {
            // If the ball is too small, remove it
            commands.entity(trigger.target()).despawn();
        }
    }
}
