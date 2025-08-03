use bevy::prelude::*;
use bevy_butler::*;

use crate::{GameStartEvent, balls::score::CommitScoreEvent, world_to_screen::WorldPos};

#[butler_plugin]
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub all_time_score: u32,
    pub points: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            all_time_score: 0,
            points: 0,
        }
    }

    pub fn reset(&mut self) {
        self.all_time_score = 0;
        self.points = 0;
    }
}

impl Default for Player {
    fn default() -> Self {
        Player::new()
    }
}

#[add_observer(plugin = PlayerPlugin)]
fn spawn_score(
    _trigger: Trigger<GameStartEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load the font
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    commands.spawn((
        Text2d::new("Dings: 0"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        WorldPos {
            translation: Vec2::new(0., 95.),
            rotation: Quat::IDENTITY,
            scale: Vec2::new(0.1, 0.1),
        },
        Player::new(), // Attach the Player component to the text entity
    ));
}

#[add_observer(plugin = PlayerPlugin)]
fn update_score(trigger: Trigger<CommitScoreEvent>, mut query: Query<(&mut Text2d, &mut Player)>) {
    let (mut text, mut player) = query.single_mut().unwrap();
    player.all_time_score += trigger.amount;
    player.points += trigger.amount;
    text.0 = format!("Dings: {}", player.points);
}
