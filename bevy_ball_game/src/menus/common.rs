use bevy::{app::AppExit, prelude::*};

use crate::app_state::AppState;

const BUTTON_BACKGROUND: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON_BACKGROUND: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON_BACKGROUND: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct AnimatedButton {
    pub bg_color: BackgroundColor,
    pub hover_color: BackgroundColor,
    pub pressed_color: BackgroundColor,
}

impl Default for AnimatedButton {
    fn default() -> Self {
        AnimatedButton {
            bg_color: BUTTON_BACKGROUND.into(),
            hover_color: HOVERED_BUTTON_BACKGROUND.into(),
            pressed_color: PRESSED_BUTTON_BACKGROUND.into(),
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn animate_buttons(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &AnimatedButton),
        Changed<Interaction>,
    >,
) {
    for (interaction, mut bg, button) in button_query.iter_mut() {
        *bg = match *interaction {
            Interaction::Pressed => button.pressed_color,
            Interaction::Hovered => button.hover_color,
            Interaction::None => button.bg_color,
        };
    }
}

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

pub fn interact_with_play_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single() {
        next_app_state.set(AppState::Game)
    }
}

pub fn interact_with_quit_button(
    button_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut exit_event: EventWriter<AppExit>,
) {
    if let Ok(Interaction::Pressed) = button_query.get_single() {
        exit_event.send(AppExit)
    }
}

pub fn add_button(parent: &mut ChildBuilder, title: String, components: impl Bundle) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(200.),
                    height: Val::Px(80.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            AnimatedButton::default(),
            components,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: title.to_string(),
                        style: TextStyle {
                            font_size: 32.,
                            color: Color::WHITE,
                            ..default()
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
}

pub fn generic_text(
    parent: &mut ChildBuilder,
    text: String,
    font_size: f32,
    components: impl Bundle,
) -> Entity {
    parent
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: text.to_string(),
                        style: TextStyle {
                            font_size,
                            color: Color::WHITE,
                            ..default()
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            },
            components,
        ))
        .id()
}

pub fn generic_image(parent: &mut ChildBuilder, image: UiImage, size: Vec2) {
    parent.spawn(ImageBundle {
        style: Style {
            width: Val::Px(size.x),
            height: Val::Px(size.y),
            margin: UiRect::new(Val::Px(8.), Val::Px(8.), Val::Px(8.), Val::Px(8.)),
            ..default()
        },
        image,
        ..default()
    });
}
