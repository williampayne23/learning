use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_butler::*;

use crate::{
    constants::{GAME_WIDTH, SNAKE_SPEED, UNIT_SIZE},
    game_state::GameOverEvent,
    handle_input::LastDirectionPressed,
};

use super::direction::Direction;

#[butler_plugin]
pub struct SnakePlugin;

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeTail;

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component)]
#[relationship(relationship_target = FollowedBy)]
struct Follows(pub Entity);

#[derive(Component, Deref)]
#[relationship_target(relationship = Follows)]
struct FollowedBy(Vec<Entity>);

#[add_system(plugin = SnakePlugin, schedule = Update)]
fn collide_snake(
    mut commands: Commands,
    snake_query: Query<(&Transform, &SnakeHead)>,
    snake_segments_query: Query<&Transform, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    for (snake_transform, _) in snake_query.iter() {
        for segment_transform in snake_segments_query.iter() {
            if (snake_transform.translation - segment_transform.translation).length()
                < UNIT_SIZE / 2.0
            {
                // Trigger the game over event
                commands.trigger(GameOverEvent);
            }
        }
    }
}

#[add_system(plugin = SnakePlugin, schedule = Update, run_if = on_timer(Duration::from_millis(SNAKE_SPEED)))]
fn move_snake(
    mut commands: Commands,
    mut snake_query: Query<(&mut Transform, &mut SnakeHead)>,
    last_direction_pressed: Res<LastDirectionPressed>,
) {
    for (mut transform, mut snake_head) in snake_query.iter_mut() {
        // Move the snake head in the last direction pressed unless it is the opposite of the current direction
        if snake_head.direction != last_direction_pressed.0.reverse() {
            snake_head.direction = last_direction_pressed.0
        }
        match snake_head.direction {
            Direction::Up => transform.translation.y += UNIT_SIZE,
            Direction::Down => transform.translation.y -= UNIT_SIZE,
            Direction::Left => transform.translation.x -= UNIT_SIZE,
            Direction::Right => transform.translation.x += UNIT_SIZE,
        }
        // Check if the snake head left the screen
        if transform.translation.x < -GAME_WIDTH / 2.0 * UNIT_SIZE
            || transform.translation.x > GAME_WIDTH / 2.0 * UNIT_SIZE
            || transform.translation.y < -GAME_WIDTH / 2.0 * UNIT_SIZE
            || transform.translation.y > GAME_WIDTH / 2.0 * UNIT_SIZE
        {
            // Trigger the game over event
            commands.trigger(GameOverEvent);
        }
    }
}

#[add_system(plugin = SnakePlugin, schedule = Update, run_if = on_timer(Duration::from_millis(SNAKE_SPEED)), after = move_snake)]
fn move_segments(
    snake_tail: Single<Entity, With<SnakeTail>>,
    snake_segments_follows: Query<&Follows, (With<SnakeSegment>, Without<SnakeHead>)>,
    mut snake_segments_transforms: Query<&mut Transform, (With<SnakeSegment>, Without<SnakeHead>)>,
    snake_head: Query<&Transform, With<SnakeHead>>,
) {
    // Get target
    let mut current = snake_tail.entity();
    while let Ok(follows) = snake_segments_follows.get(current) {
        if let Ok([mut target_transform, follows_transform]) =
            snake_segments_transforms.get_many_mut([current, follows.0])
        {
            // Get the transform of the entity that this segment follows
            target_transform.translation = follows_transform.translation;
            current = follows.0;
        } else if let Ok([mut target_transform]) = snake_segments_transforms.get_many_mut([current])
        {
            if let Ok(snake_head) = snake_head.get(follows.0) {
                // Get the transform of the entity that this segment follows
                target_transform.translation = snake_head.translation;
            }
            return;
        }
    }
}
