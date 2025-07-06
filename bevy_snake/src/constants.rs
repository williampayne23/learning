use bevy::prelude::*;

/**
* The Size of the game window in units.
*/
pub const GAME_WIDTH: f32 = 21.0;

/**
* Snake speed in milliseconds.
*/
pub const SNAKE_SPEED: u64 = 100;

/**
* Unit size of the snake in pixels.
*/
pub const UNIT_SIZE: f32 = 40.0;
pub const FOOD_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
pub const SNAKE_HEAD_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
pub const FOOD_BUBBLE_HEAD_COLOR: Color = SNAKE_HEAD_COLOR;
pub const SNAKE_BODY_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const FOOD_BUBBLE_BODY_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
