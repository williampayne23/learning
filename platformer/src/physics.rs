use bevy::prelude::*;
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Rigidbody>()
            .add_systems(PreUpdate, rigidbody_update);
    }
}

#[derive(Reflect, Component, Default)]
pub struct Rigidbody {
    pub mass: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    //Acceleration and drag are different because they let us work out whether to clamp to zero
    pub drag: Vec3,
    pub max_v: Option<f32>,
}

impl Rigidbody {
    pub fn from_max_v(max_v: f32) -> Self {
        Rigidbody {
            max_v: Some(max_v),
            ..default()
        }
    }
}

fn rigidbody_update(mut rigidbody_query: Query<(&mut Rigidbody, &mut Transform)>, time: Res<Time>) {
    for (mut rigidbody, mut transform) in rigidbody_query.iter_mut() {
        let mut new_velocity = rigidbody.velocity + rigidbody.drag * time.delta_seconds();
        if (new_velocity.x * rigidbody.velocity.x).is_sign_negative() {
            new_velocity.x = 0.0;
        }
        if (new_velocity.y * rigidbody.velocity.y).is_sign_negative() {
            new_velocity.y = 0.0;
        }
        new_velocity += rigidbody.acceleration * time.delta_seconds();
        if let Some(max_v) = rigidbody.max_v {
            new_velocity.x = new_velocity.x.clamp(-max_v, max_v);
        }
        rigidbody.velocity = new_velocity;
        transform.translation += rigidbody.velocity * time.delta_seconds();
        rigidbody.acceleration = Vec3::ZERO;
        rigidbody.drag = Vec3::ZERO;
    }
}
