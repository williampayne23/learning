use bevy::{prelude::*, window::PrimaryWindow};

use super::tick_schedule::MovementSet;

pub struct CollidePlugin;
impl Plugin for CollidePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>().add_systems(
            Update,
            (constrain_to_window, collision).in_set(MovementSet::CollisionChecking),
        );
    }
}

#[derive(Component)]
pub struct Collides {
    pub object_size: f32,
}

#[derive(Event)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Option<Entity>,
    pub direction: Vec3,
}

fn constrain_to_window(
    mut ev_constrain: EventWriter<CollisionEvent>,
    mut constraint_query: Query<(Entity, &mut Transform, &Collides)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (entity, mut transform, constrain) in constraint_query.iter_mut() {
        let half_object_size = constrain.object_size / 2.0;
        let x_min = 0.0 + half_object_size;
        let x_max = window.width() - half_object_size;
        let y_min = 0.0 + half_object_size;
        let y_max = window.height() - half_object_size;
        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min;
            ev_constrain.send(CollisionEvent {
                entity_a: entity,
                entity_b: None,
                direction: Vec3 {
                    x: -1.0,
                    y: 0.0,
                    z: 0.0,
                },
            })
        } else if translation.x > x_max {
            translation.x = x_max;
            ev_constrain.send(CollisionEvent {
                entity_a: entity,
                entity_b: None,
                direction: Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            })
        }

        if translation.y < y_min {
            translation.y = y_min;
            ev_constrain.send(CollisionEvent {
                entity_a: entity,
                entity_b: None,
                direction: Vec3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
            })
        } else if translation.y > y_max {
            translation.y = y_max;
            ev_constrain.send(CollisionEvent {
                entity_a: entity,
                entity_b: None,
                direction: Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            })
        }
        transform.translation = translation;
    }
}

pub fn collision(
    mut ev: EventWriter<CollisionEvent>,
    collision_query: Query<(Entity, &Transform, &Collides)>,
) {
    for (entity_a, transform_a, collider_a) in collision_query.iter() {
        for (entity_b, transform_b, collider_b) in collision_query.iter() {
            if entity_a == entity_b {
                continue;
            }
            let distance = transform_a.translation.distance(transform_b.translation);
            let direction = transform_a.translation - transform_b.translation;
            if distance < (collider_a.object_size + collider_b.object_size) / 2.0 {
                ev.send(CollisionEvent {
                    entity_a,
                    entity_b: Some(entity_b),
                    direction,
                });
            }
        }
    }
}
