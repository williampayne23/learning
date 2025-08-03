use crate::balls::{Ball, BallHitWallEvent, BallsPlugin};
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct ExpBounceDecay(pub f32);

#[add_observer(plugin = BallsPlugin)]
fn ensure_exp_decay_rate_is_not_positive(
    trigger: Trigger<OnAdd, ExpBounceDecay>,
    mut query: Query<&mut ExpBounceDecay>,
) {
    if let Some(mut decay_rate) = query.get_mut(trigger.target()).ok() {
        if decay_rate.0 > 1.0 {
            decay_rate.0 = 1.0
        }
    }
}

#[add_observer(plugin = BallsPlugin)]
fn exp_shrink_on_bounce(
    trigger: Trigger<BallHitWallEvent>,
    mut commands: Commands,
    mut ball_query: Query<(&mut Ball, &ExpBounceDecay)>,
) {
    if let Some((mut ball, decay_rate)) = ball_query.get_mut(trigger.target()).ok() {
        ball.diameter *= decay_rate.0;
        // Shrink the ball by rate on each bounce
        if ball.diameter < 1.0 {
            // If the ball is too small, remove it
            commands.entity(trigger.target()).despawn();
        }
    }
}

#[derive(Component)]
pub struct LinearBounceDecay(pub f32);

#[add_observer(plugin = BallsPlugin)]
fn ensure_linear_decay_is_positive(
    trigger: Trigger<OnAdd, LinearBounceDecay>,
    mut query: Query<&mut LinearBounceDecay>,
) {
    if let Some(mut decay_rate) = query.get_mut(trigger.target()).ok() {
        if decay_rate.0 < 0. {
            decay_rate.0 = 0.
        }
    }
}

#[add_observer(plugin = BallsPlugin)]
fn linear_shrink_on_bounce(
    trigger: Trigger<BallHitWallEvent>,
    mut commands: Commands,
    mut ball_query: Query<(&mut Ball, &LinearBounceDecay)>,
) {
    if let Some((mut ball, decay_rate)) = ball_query.get_mut(trigger.target()).ok() {
        ball.diameter -= decay_rate.0;
        // Shrink the ball by rate on each bounce
        if ball.diameter < 1.0 {
            // If the ball is too small, remove it
            commands.entity(trigger.target()).despawn();
        }
    }
}
