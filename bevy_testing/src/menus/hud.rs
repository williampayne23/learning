use bevy::prelude::*;

use crate::{
    app_state::AppState,
    game::{enemy::Enemy, game_logic::Score},
};

use super::common::{generic_image, generic_text};
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_hud)
            .add_systems(OnExit(AppState::Game), despawn_hud)
            .add_systems(
                Update,
                (update_score, update_enemies).run_if(in_state(AppState::Game)),
            );
    }
}
#[derive(Component)]
pub struct Hud;

#[derive(Component)]
pub struct ScoreIndicator;

#[derive(Component)]
pub struct EnemyIndicator;

pub fn update_score(
    mut score_query: Query<&mut Text, With<ScoreIndicator>>,
    score_res: Res<Score>,
) {
    if score_res.is_changed() {
        if let Ok(mut score_indicator) = score_query.get_single_mut() {
            score_indicator.sections = vec![TextSection {
                value: score_res.value.to_string(),
                style: TextStyle {
                    font_size: 32.,
                    color: Color::WHITE,
                    ..default()
                },
            }]
        }
    }
}

pub fn update_enemies(
    mut enemy_indicator_query: Query<&mut Text, With<EnemyIndicator>>,
    enemy_q: Query<(), With<Enemy>>,
) {
    let enemies = enemy_q.iter().len();

    if let Ok(mut enemy_indicator) = enemy_indicator_query.get_single_mut() {
        enemy_indicator.sections = vec![TextSection {
            value: enemies.to_string(),
            style: TextStyle {
                font_size: 32.,
                color: Color::WHITE,
                ..default()
            },
        }]
    }
}

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_hud(&mut commands, &asset_server);
    println!("Spawning Main Menu");
}

pub fn despawn_hud(mut commands: Commands, menu_query: Query<Entity, With<Hud>>) {
    if let Ok(hud) = menu_query.get_single() {
        commands.entity(hud).despawn_recursive();
        println!("Despawning Main Menu");
    }
}

pub fn build_hud(commands: &mut Commands, _asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            Hud,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(80.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    background_color: Color::rgba(0.15, 0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    generic_image(
                        parent,
                        _asset_server.load("sprites/star.png").into(),
                        Vec2::ONE * 32.,
                    );
                    generic_text(parent, "0".to_string(), 64., ScoreIndicator);
                });
            parent.spawn(NodeBundle {
                style: Style {
                    flex_grow: 1.,
                    ..default()
                },
                ..default()
            });
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(150.),
                        height: Val::Px(80.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    background_color: Color::rgba(0.15, 0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    generic_text(parent, "4".to_string(), 64., EnemyIndicator);
                    generic_image(
                        parent,
                        _asset_server.load("sprites/ball_red_large.png").into(),
                        Vec2::ONE * 32.,
                    );
                });
        });
}
