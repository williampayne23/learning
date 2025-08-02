use crate::balls::{Ball, BallHitWallEvent, BallsPlugin, WallDirection};
use bevy::prelude::*;
use bevy_butler::*;

#[derive(Component)]
pub struct PreferredWall {
    pub wall_direction: WallDirection,
}

// #[add_observer(plugin = BallsPlugin)]
// fn extra_points_if_wall_direction_is_correct() {}
