use crate::balls::{Ball, BallHitWallEvent, BallsPlugin, WallDirection};
use crate::world_to_screen::{WorldPos, WorldToScreenConfig};
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec2,
}

impl Velocity {
    pub fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }

    pub fn random_unit() -> Self {
        let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        Velocity::from(Vec2::new(angle.cos(), angle.sin()))
    }
}

// Move ball system
#[add_system(plugin = BallsPlugin, schedule = Update)]
fn move_ball(
    mut query: Query<(Entity, &mut WorldPos, &mut Velocity, &Ball)>,
    world_to_screen_config: Res<WorldToScreenConfig>,
    mut commands: Commands,
) {
    let world_size = Vec2::new(world_to_screen_config.width, world_to_screen_config.height);

    for (entity, mut world_pos, mut velocity, ball) in query.iter_mut() {
        // Here you would implement the logic to move the balls
        // For example, you could update the translation based on some velocity
        world_pos.translation += velocity.velocity;

        // Collision detection against world_size
        let ball_size = Vec2::new(ball.diameter / 2.0, ball.diameter / 2.0);
        if world_pos.translation.x < ball_size.x
            || world_pos.translation.x > world_size.x - ball_size.x
        {
            if world_pos.translation.x < ball_size.x {
                commands.trigger_targets(
                    BallHitWallEvent {
                        wall_direction: WallDirection::Left,
                    },
                    entity,
                );
            } else {
                commands.trigger_targets(
                    BallHitWallEvent {
                        wall_direction: WallDirection::Right,
                    },
                    entity,
                );
            }
            // Clamp
            world_pos.translation.x = world_pos
                .translation
                .x
                .clamp(ball_size.x, world_size.x - ball_size.x);
            // Reverse the x velocity
            velocity.velocity.x *= -1.0;
        }

        if world_pos.translation.y < ball_size.y
            || world_pos.translation.y > world_size.y - ball_size.y
        {
            if world_pos.translation.y < ball_size.y {
                commands.trigger_targets(
                    BallHitWallEvent {
                        wall_direction: WallDirection::Top,
                    },
                    entity,
                );
            } else {
                commands.trigger_targets(
                    BallHitWallEvent {
                        wall_direction: WallDirection::Bottom,
                    },
                    entity,
                );
            }

            // Clamp
            world_pos.translation.y = world_pos
                .translation
                .y
                .clamp(ball_size.y, world_size.y - ball_size.y);

            // Reverse the y velocity
            velocity.velocity.y *= -1.0;
        }
    }
}
