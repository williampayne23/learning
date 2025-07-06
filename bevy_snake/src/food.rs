use bevy::prelude::*;
use bevy_butler::*;

use super::constants::{FOOD_COLOR, GAME_WIDTH, UNIT_SIZE};

#[butler_plugin]
pub struct FoodPlugin;

#[derive(Debug, Event)]
pub struct SpawnFoodEvent();

#[derive(Debug, Event)]
pub struct EatFoodEvent();

#[derive(Component)]
pub struct Food;

use super::snake::SnakeHead;

#[add_observer(plugin = FoodPlugin)]
pub fn food_spawner(
    _trigger: Trigger<SpawnFoodEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn food at a random position
    let random_x = rand::random::<f32>();
    let random_y = rand::random::<f32>();
    let grid_x = (random_x * GAME_WIDTH - GAME_WIDTH / 2.0).trunc();
    let grid_y = (random_y * GAME_WIDTH - GAME_WIDTH / 2.0).trunc();
    let x = grid_x * UNIT_SIZE;
    let y = grid_y * UNIT_SIZE;
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(UNIT_SIZE, UNIT_SIZE))),
        MeshMaterial2d(materials.add(FOOD_COLOR)),
        Transform::from_xyz(x, y, 0.),
        Food,
    ));
}

#[add_system(plugin = FoodPlugin, schedule = Update)]
fn collide_food(
    mut commands: Commands,
    snake_query: Query<(&Transform, &SnakeHead)>,
    food_query: Query<(Entity, &Transform), With<Food>>,
) {
    for (snake_transform, _) in snake_query.iter() {
        for (food_entity, food_transform) in food_query.iter() {
            if (snake_transform.translation - food_transform.translation).length() < UNIT_SIZE / 2.0
            {
                commands.entity(food_entity).despawn();
                commands.trigger(EatFoodEvent());
                commands.trigger(SpawnFoodEvent());
            }
        }
    }
}
