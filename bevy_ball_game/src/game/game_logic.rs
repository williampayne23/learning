use bevy::{
    audio::{Volume, VolumeLevel},
    prelude::*,
    window::PrimaryWindow,
};

use crate::app_state::AppState;

use super::{
    collide::CollisionEvent, enemy::Enemy, player::Player, star::Star, tick_schedule::Simulation,
};

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScores>()
            .add_event::<GameOverEvent>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    death,
                    handle_game_over,
                    debug_high_scores_updated,
                    collect_star,
                    print_score,
                )
                    .in_set(Simulation),
            );
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Debug, Default)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>,
}

#[derive(Event)]
pub struct GameOverEvent {}

pub fn death(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut ev_collide: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut ev_game_over: EventWriter<GameOverEvent>,
) {
    for ev in ev_collide.iter() {
        if let Ok(player) = player_query.get(ev.entity_a) {
            if let Some(entity_b) = ev.entity_b {
                if enemy_query.get(entity_b).is_ok() {
                    command.entity(player).despawn();
                    let end_sound = asset_server.load("audio/explosionCrunch_000.ogg");
                    command.spawn(AudioBundle {
                        source: end_sound,
                        settings: PlaybackSettings {
                            volume: Volume::Relative(VolumeLevel::new(0.1)),
                            ..default()
                        },
                    });
                    ev_game_over.send(GameOverEvent {});
                }
            }
        }
    }
}

pub fn collect_star(
    mut command: Commands,
    asset_server: Res<AssetServer>,
    mut ev_collide: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    star_query: Query<Entity, With<Star>>,
    mut score: ResMut<Score>,
) {
    for ev in ev_collide.iter() {
        if player_query.get(ev.entity_a).is_ok() {
            if let Some(entity_b) = ev.entity_b {
                if let Ok(star) = star_query.get(entity_b) {
                    command.entity(star).despawn();
                    let star_sound = asset_server.load("audio/laserLarge_000.ogg");
                    command.spawn(AudioBundle {
                        source: star_sound,
                        settings: PlaybackSettings {
                            volume: Volume::Relative(VolumeLevel::new(0.1)),
                            ..default()
                        },
                    });
                    score.value += 1;
                }
            }
        }
    }
}

pub fn print_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value)
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    mut ev_game_over: EventReader<GameOverEvent>,
    score: Res<Score>,
    mut high_scores: ResMut<HighScores>,
) {
    for _ in ev_game_over.iter() {
        high_scores.scores.push(("Player".to_string(), score.value));
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}

pub fn debug_high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("{:?}", high_scores)
    }
}
