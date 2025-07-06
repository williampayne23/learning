use bevy::prelude::*;
use bevy_butler::*;

#[butler_plugin]
pub struct GameStatePlugin;

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Event)]
pub struct GameResetEvent;

#[derive(Event)]
pub struct GameStartEvent;
