use bevy::prelude::*;

use crate::app_state::AppState;

use super::common::{add_button, generic_image, generic_text, PlayButton, QuitButton};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}

#[derive(Component)]
pub struct MainMenu;

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
    println!("Spawning Main Menu");
}

pub fn despawn_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu) = menu_query.get_single() {
        commands.entity(main_menu).despawn_recursive();
        println!("Despawning Main Menu");
    }
}

pub fn build_main_menu(commands: &mut Commands, _asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
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
            MainMenu,
        ))
        .with_children(|parent| {
            // TITLE
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        padding: UiRect::bottom(Val::Px(100.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    //Image 1
                    generic_image(
                        parent,
                        _asset_server.load("sprites/ball_blue_large.png").into(),
                        Vec2::ONE * 64.,
                    );
                    //Text
                    generic_text(parent, "Bevy Ball Game".into(), 64., ());
                    //Image 2
                    generic_image(
                        parent,
                        _asset_server.load("sprites/ball_red_large.png").into(),
                        Vec2::ONE * 64.,
                    );
                });
            // PLAY BUTTON
            add_button(parent, "Play".to_string(), PlayButton);
            // QUIT BUTTON
            add_button(parent, "Quit".to_string(), QuitButton)
        })
        .id();
    main_menu_entity
}
