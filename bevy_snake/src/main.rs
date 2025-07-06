use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};

mod food;
use food::{EatFoodEvent, FoodPlugin};

mod food_bubble;
use food_bubble::FoodBubblePlugin;

mod constants;

use constants::{GAME_WIDTH, UNIT_SIZE};

mod direction;
use direction::Direction;

mod handle_input;
use handle_input::HandleInputPlugin;

mod game_state;
use game_state::GameStatePlugin;

mod snake;
use snake::SnakePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake Game".to_string(),
                    resolution: WindowResolution::new(
                        UNIT_SIZE * (GAME_WIDTH + 4.0),
                        UNIT_SIZE * (GAME_WIDTH + 4.0),
                    )
                    .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_plugins((
            FoodPlugin,
            FoodBubblePlugin,
            HandleInputPlugin,
            GameStatePlugin,
            SnakePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (draw_gizmos, update_text, spawn_snake_tail).chain())
        .add_systems(
            PostUpdate,
            (ApplyDeferred, despawn_entities, ApplyDeferred).chain(),
        )
        .add_observer(score)
        .run();
}

#[derive(Component)]
struct Score(u32);

impl Score {
    fn new() -> Self {
        Score(0)
    }
}

#[derive(Component)]
struct HighScore(u32);

impl HighScore {
    fn new() -> Self {
        HighScore(0)
    }
}

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakeTail;

/// This is a "relationship" component.
/// Add it to an entity that "likes" another entity.
#[derive(Component)]
#[relationship(relationship_target = FollowedBy)]
struct Follows(pub Entity);

/// This is the "relationship target" component.
/// It will be automatically inserted and updated to contain
/// all entities that currently "like" this entity.
#[derive(Component, Deref)]
#[relationship_target(relationship = Follows)]
struct FollowedBy(Vec<Entity>);

#[derive(Component)]
struct SnakeSegment;

#[derive(Component)]
struct Despawn;

fn despawn_entities(mut commands: Commands, despawn_queries: Query<Entity, With<Despawn>>) {
    for entity in despawn_queries.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_snake_tail(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    snake_query: Query<(Entity, &Transform), (With<SnakeTail>, With<food_bubble::FoodBubble>)>,
) {
    for (entity, transform) in snake_query.iter() {
        // Drop the old tail component
        commands.entity(entity).remove::<SnakeTail>();
        commands.entity(entity).remove::<food_bubble::FoodBubble>();

        // Spawn a new tail component following the old one
        let color = Color::srgb(0.5, 0.5, 0.5);
        commands.spawn((
            Name::new("Snake Tail"),
            Mesh2d(meshes.add(Rectangle::new(UNIT_SIZE, UNIT_SIZE))),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(-10000.0, transform.translation.y, 0.), // Spawn off-screen
            SnakeTail,
            SnakeSegment,
            Follows(entity),
        ));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the font
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    // Spawn the camera
    commands.spawn(Camera2d);

    commands.spawn((
        Text2d::new("Score: 0"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_xyz(-300., 700., 0.),
        Score::new(),
    ));
    commands.spawn((
        Text2d::new("High Score: 0"),
        Transform::from_xyz(300., 700., 0.),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Center),
        HighScore::new(),
    ));
}

fn score(
    _: Trigger<EatFoodEvent>,
    mut scores: Query<&mut Score>,
    mut high_scores: Query<&mut HighScore>,
) {
    for mut score in scores.iter_mut() {
        score.0 += 1;
    }
    for mut high_score in high_scores.iter_mut() {
        if let Ok(score) = scores.single() {
            if score.0 > high_score.0 {
                high_score.0 = score.0;
            }
        }
    }
}

fn update_text(
    mut text_query: Query<(&mut Text2d, &mut Score), Without<HighScore>>,
    mut high_score_query: Query<(&mut Text2d, &mut HighScore), Without<Score>>,
) {
    for (mut text, score) in text_query.iter_mut() {
        text.0 = format!("Score: {}", score.0);
    }
    for (mut text, high_score) in high_score_query.iter_mut() {
        text.0 = format!("High Score: {}", high_score.0);
    }
}

fn draw_gizmos(mut gizmos: Gizmos) {
    // Draw a rectangle at the center of the screen
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(GAME_WIDTH as u32, GAME_WIDTH as u32),
            Vec2::new(UNIT_SIZE, UNIT_SIZE),
            // Dark gray
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}
