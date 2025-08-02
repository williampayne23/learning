use bevy::prelude::*;
use bevy_butler::*;
use movement::Velocity;

use crate::GameStartEvent;
use crate::world_to_screen::WorldPos;

mod movement;
mod properties;

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

// Spawn ball system
#[add_observer(plugin = BallsPlugin)]
pub fn spawn_first_ball(_trigger: Trigger<GameStartEvent>, mut commands: Commands) {
    commands.spawn(Ball { diameter: 10.0 });
}
