use bevy::audio::{Volume, VolumeLevel};
use bevy::prelude::*;
// use bevy::ui::debug;
use bevy::window::PrimaryWindow;
use rand::random;

use crate::app_state::AppState;

use super::collide::{Collides, CollisionEvent};
use super::tick_schedule::{GameStart, MovementSet, Simulation};
use super::DespawnOnEnd;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>().add_systems(
            Update,
            (
                (enemy_spawn_timer_tick, spawn_enemy_over_time).in_set(Simulation),
                enemy_movement.in_set(MovementSet::Movement),
                enemy_bounce.in_set(MovementSet::Direction),
            ),
        );
    }
}

pub const ENEMY_SPEED: f32 = 500.0;
pub const ENEMY_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Enemy {
    direction: Vec3,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        }
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = enemy.direction;
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn enemy_bounce(
    mut commands: Commands,
    mut ev_constrain: EventReader<CollisionEvent>,
    mut mut_enemy_query: Query<&mut Enemy>,
    enemy_query: Query<Entity, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_constrain.iter() {
        if let Ok(mut enemy) = mut_enemy_query.get_mut(ev.entity_a) {
            if let Some(entity_b) = ev.entity_b {
                if enemy_query.get(entity_b).is_ok() {
                    enemy.direction = ev.direction.normalize();
                    play_bounce(&mut commands, &asset_server);
                }
            } else {
                match ev.direction {
                    v if v.x == 0. => enemy.direction.y = -enemy.direction.y,
                    v if v.y == 0. => enemy.direction.x = -enemy.direction.x,
                    _ => {}
                }
                play_bounce(&mut commands, &asset_server);
            }
        }
    }
}

fn play_bounce(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
    let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
    // Randomly play one of the two sound effects.
    let sound_effect = if random::<f32>() > 0.5 {
        sound_effect_1
    } else {
        sound_effect_2
    };

    let playback_settings = PlaybackSettings {
        volume: Volume::Relative(VolumeLevel::new(0.1)),
        ..default()
    };

    commands.spawn(AudioBundle {
        source: sound_effect,
        settings: playback_settings,
    });
}

pub fn spawn_enemy(commands: &mut Commands, window: &Window, asset_server: &Res<AssetServer>) {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(random_x, random_y, 0.0),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Enemy {
            direction: Vec3::new(random::<f32>() - 0.5, random::<f32>() - 0.5, 0.0).normalize(),
        },
        Collides {
            object_size: ENEMY_SIZE,
        },
        DespawnOnEnd,
    ));
}

pub fn enemy_spawn_timer_tick(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemy_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    let window = window_query.get_single().unwrap();
    if enemy_spawn_timer.timer.finished() {
        spawn_enemy(&mut commands, window, &asset_server)
    }
}
