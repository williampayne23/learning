use bevy::prelude::*;
use bevy_butler::*;
use rand;

use crate::GameStartEvent;
use crate::world_to_screen::{WorldPos, WorldToScreenConfig};

#[butler_plugin]
pub struct BallsPlugin;

#[derive(Component)]
pub struct Ball {
    pub diameter: f32,
    pub decay_rate: f32,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WallDirection {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Event)]
pub struct BallHitWallEvent {
    pub wall_direction: WallDirection,
}

#[add_observer(plugin = BallsPlugin)]
fn on_add_ball(
    trigger: Trigger<OnAdd, Ball>,
    mut commands: Commands,
    query: Query<&Ball>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let ball = query.get(trigger.target()).unwrap();
    commands.entity(trigger.target()).insert((
        Mesh2d(meshes.add(Rectangle::new(ball.diameter, ball.diameter))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.8, 0.2))),
        WorldPos::from_translation(Vec2::new(50., 50.)), // Default position
        Velocity::random_unit(),                         // Random velocity
    ));
}

#[add_system(plugin = BallsPlugin, schedule = PostUpdate)]
fn shrink_mesh_on_ball_change(
    mut commands: Commands,
    query: Query<(Entity, &Ball), Changed<Ball>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // When the ball is changed we update the mesh to match the new diameter
    for (entity, ball) in query.iter() {
        commands.entity(entity).insert(Mesh2d(
            meshes.add(Rectangle::new(ball.diameter, ball.diameter)),
        ));
    }
}

#[add_observer(plugin = BallsPlugin)]
fn shrink_on_bounce(
    trigger: Trigger<BallHitWallEvent>,
    mut commands: Commands,
    mut ball_query: Query<&mut Ball>,
) {
    let mut ball = ball_query.get_mut(trigger.target()).unwrap();
    // Shrink the ball by 10% on each bounce
    ball.diameter *= ball.decay_rate;
    if ball.diameter < 1.0 {
        // If the ball is too small, remove it
        commands.entity(trigger.target()).despawn();
    }
}

// Spawn ball system
#[add_observer(plugin = BallsPlugin)]
pub fn ball_spawner(_trigger: Trigger<GameStartEvent>, mut commands: Commands) {
    commands.spawn((Ball {
        diameter: 10.0,
        decay_rate: 1.0,
    },));
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
