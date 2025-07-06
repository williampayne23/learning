use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::app_state::AppState;

use super::collide::Collides;
use super::tick_schedule::Simulation;
use super::DespawnOnEnd;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                (star_spawn_timer_tick, spawn_stars_over_time).in_set(Simulation),
            );
    }
}

pub const NUM_STARS: i32 = 4;
pub const STAR_SIZE: f32 = 30.;

#[derive(Resource)]
pub struct StarSpawnTimer {
    timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer {
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
pub struct Star {}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUM_STARS {
        spawn_star(&mut commands, window, &asset_server)
    }
}

pub fn star_spawn_timer_tick(mut star_spawn_time: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_time.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    star_spawn_timer: Res<StarSpawnTimer>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if star_spawn_timer.timer.finished() {
        spawn_star(&mut commands, window, &asset_server);
    }
}

fn spawn_star(commands: &mut Commands, window: &Window, asset_server: &Res<AssetServer>) {
    let sprite = asset_server.load("sprites/star.png");
    let x_pos = random::<f32>() * window.width();
    let y_pos = random::<f32>() * window.height();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x_pos, y_pos, 0.0),
            texture: sprite,
            ..default()
        },
        Star {},
        Collides {
            object_size: STAR_SIZE,
        },
        DespawnOnEnd,
    ));
}
