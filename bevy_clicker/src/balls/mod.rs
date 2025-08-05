use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_butler::*;
use movement::Velocity;
use properties::bounce_decaying::LinearBounceDecay;
use properties::bounce_score::ScoreOnWallBounce;
use properties::preferred_wall::PreferredWall;

use crate::GameStartEvent;
use crate::input::mouse::MouseClickEvent;
use crate::world_to_screen::{WorldPos, WorldToScreenConfig, get_proportional_size};

mod movement;
mod properties;
pub mod score;

#[butler_plugin]
pub struct BallsPlugin;

#[derive(Component)]
pub struct Ball {
    pub diameter: f32,
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
fn required_ball_components(
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
        Velocity::random_unit(), // Random velocity
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

// Spawn ball system
#[add_observer(plugin = BallsPlugin)]
pub fn spawn_first_ball(_trigger: Trigger<GameStartEvent>, mut commands: Commands) {
    commands.spawn((
        Ball { diameter: 5.0 },
        ScoreOnWallBounce { score: 1. },
        LinearBounceDecay(1.),
        PreferredWall {
            wall_direction: WallDirection::Top,
            score_multiplier: 2.,
        },
        WorldPos::from_translation(Vec2::new(50., 50.)), // Default position
    ));
}

// #[add_observer(plugin = BallsPlugin)]
pub fn spawn_on_mouse_click(
    trigger: Trigger<MouseClickEvent>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    world_to_screen_config: Res<WorldToScreenConfig>,
) {
    let primary_window = window_query.single().expect("No primary window found");
    let window_size = primary_window.resolution.physical_size().as_vec2();
    let window_scale = primary_window.scale_factor();
    let logical_size = Vec2::new(
        window_size.x as f32 / window_scale,
        window_size.y as f32 / window_scale,
    );
    let world_size = Vec2::new(world_to_screen_config.width, world_to_screen_config.height);
    let proportional_screen_size = get_proportional_size(&world_size, &logical_size);

    // Spawn a ball at the position of the mouse click
    commands.spawn((
        Ball { diameter: 5.0 },
        ScoreOnWallBounce { score: 1. },
        LinearBounceDecay(1.),
        WorldPos::from_screen_pos(&trigger.pos, &world_size, &proportional_screen_size),
    ));
}
