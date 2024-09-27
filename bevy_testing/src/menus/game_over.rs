use super::common::{add_button, generic_text, PlayButton, QuitButton};
use crate::{app_state::AppState, game::game_logic::Score};
use bevy::prelude::*;

pub struct GameOverMenuPlugin;

impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over)
            .add_systems(OnExit(AppState::GameOver), despawn_game_over)
            .add_systems(
                Update,
                interact_with_main_menu_button.run_if(in_state(AppState::GameOver)),
            );
    }
}

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct MainMenuButton;

pub fn interact_with_main_menu_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<MainMenuButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single() {
        next_app_state.set(AppState::MainMenu)
    }
}

fn spawn_game_over(mut commands: Commands, score: Res<Score>) {
    build_game_over(&mut commands, &score);
    println!("Spawning Game Over")
}

fn despawn_game_over(mut commands: Commands, menu_query: Query<Entity, With<GameOverMenu>>) {
    if let Ok(game_over_menu) = menu_query.get_single() {
        commands.entity(game_over_menu).despawn_recursive();
        println!("Despawning Game Over");
    }
}

pub fn build_game_over(commands: &mut Commands, score: &Res<Score>) -> Entity {
    let game_over_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(8.),
                    ..default()
                },
                // background_color: Color::RED.into(),
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            //Text
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::bottom(Val::Px(100.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    generic_text(parent, format!("Final score: {}", score.value), 64., ());
                });
            // PLAY BUTTON
            add_button(parent, "Main Menu".to_string(), MainMenuButton);
            add_button(parent, "Restart".to_string(), PlayButton);
            add_button(parent, "Quit".to_string(), QuitButton);
        })
        .id();
    game_over_entity
}
