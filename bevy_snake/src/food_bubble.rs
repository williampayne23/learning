use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_butler::*;
use std::time::Duration;

use super::constants::{
    FOOD_BUBBLE_BODY_COLOR, FOOD_BUBBLE_HEAD_COLOR, SNAKE_BODY_COLOR, SNAKE_HEAD_COLOR, SNAKE_SPEED,
};
use super::food::EatFoodEvent;
use super::{FollowedBy, SnakeHead};

#[butler_plugin]
pub struct FoodBubblePlugin;

#[derive(Component)]
pub struct FoodBubble;

#[add_observer(plugin = FoodBubblePlugin)]
fn food_bubble_changes_snake_color(
    trigger: Trigger<OnInsert, FoodBubble>,
    head_food_bubble: Query<Entity, With<SnakeHead>>,
    tail_food_bubble: Query<Entity, Without<SnakeHead>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let target = trigger.target();
    if let Ok(head) = head_food_bubble.get(target) {
        commands
            .entity(head)
            .remove::<MeshMaterial2d<ColorMaterial>>()
            .insert(MeshMaterial2d(materials.add(FOOD_BUBBLE_HEAD_COLOR)));
    } else if let Ok(body) = tail_food_bubble.get(target) {
        commands
            .entity(body)
            .remove::<MeshMaterial2d<ColorMaterial>>()
            .insert(MeshMaterial2d(materials.add(FOOD_BUBBLE_BODY_COLOR)));
    }
}

#[add_observer(plugin = FoodBubblePlugin)]
fn food_bubble_unchanges_snake_colors(
    trigger: Trigger<OnRemove, FoodBubble>,
    head_food_bubble: Query<Entity, With<SnakeHead>>,
    body_food_bubble: Query<Entity, Without<SnakeHead>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    info!("Changing colour for removed food bubble");
    let target = trigger.target();
    if let Ok(head_entity) = head_food_bubble.get(target) {
        commands
            .entity(head_entity)
            .remove::<MeshMaterial2d<ColorMaterial>>()
            .insert(MeshMaterial2d(materials.add(SNAKE_HEAD_COLOR)));
    } else if let Ok(body_entity) = body_food_bubble.get(target) {
        commands
            .entity(body_entity)
            .remove::<MeshMaterial2d<ColorMaterial>>()
            .insert(MeshMaterial2d(materials.add(SNAKE_BODY_COLOR)));
    }
}

#[add_observer(plugin = FoodBubblePlugin)]
fn insert_food_bubble_on_eat(
    _: Trigger<EatFoodEvent>,
    mut commands: Commands,
    head_query: Query<Entity, (With<SnakeHead>, Without<FoodBubble>)>,
) {
    for entity in head_query.iter() {
        // Spawn a new food bubble component following the head
        commands.entity(entity).insert(FoodBubble);
    }
}

#[add_system(plugin = FoodBubblePlugin, schedule = Update, run_if = on_timer(Duration::from_millis(SNAKE_SPEED)))]
fn propogate_food_bubble(
    mut commands: Commands,
    food_query: Query<(Entity, &FollowedBy), With<FoodBubble>>,
) {
    for (entity, followed_by) in food_query.iter() {
        // Remove the old food bubble component
        info!("Removing food bubble from entity {:?}", entity);
        commands.entity(entity).remove::<FoodBubble>();
        for entity in followed_by.iter() {
            commands.entity(entity).insert(FoodBubble);
        }
    }
}
