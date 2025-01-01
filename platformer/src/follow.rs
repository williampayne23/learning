use bevy::{prelude::*, window::PrimaryWindow};

use crate::{camera::MainCamera, physics::Rigidbody};
pub struct FollowTargetPlugin;

impl Plugin for FollowTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                followers_follow_target,
                my_cursor_system,
                follower_target_gizmo,
            ),
        )
        .register_type::<Follower>();
        // .add_systems(Startup, insert_follower);
    }
}

#[derive(Component, Reflect, Default)]
pub struct Follower {
    pub target: Option<Vec3>,
    pub max_v: Vec3,
    pub max_d: Vec3,
    pub max_a: Vec3,
    pub target_area: f32,
    pub deadzone: Vec3,
    pub target_last: Option<Vec3>,
}

fn my_cursor_system(
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_input: Res<Input<MouseButton>>,
    mut follower_q: Query<&mut Follower>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so Query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            for mut f in follower_q.iter_mut() {
                f.target = Some(Vec3 {
                    x: world_position.x,
                    y: world_position.y,
                    z: 0.0,
                });
            }
        }
    }
}

fn followers_follow_target(
    mut gizmos: Gizmos,
    mut followers_query: Query<(&mut Rigidbody, &mut Transform, &mut Follower)>,
    time: Res<Time>,
) {
    if let Ok((mut rigidbody, mut transform, mut follower)) = followers_query.get_single_mut() {
        if let Some(mut target) = follower.target {
            let target_velocity = if let Some(last) = follower.target_last {
                target - last
            } else {
                Vec3::ZERO
            };
            follower.target_last = follower.target;
            if rigidbody.velocity.length() <= 0.01 {
                //If just starting clamp if in deadzone.
                if (target.x - transform.translation.x).abs() < follower.deadzone.x {
                    target.x = transform.translation.x;
                }
                if (target.y - transform.translation.y).abs() < follower.deadzone.y {
                    target.y = transform.translation.y;
                }
                if (target.z - transform.translation.z).abs() < follower.deadzone.z {
                    target.z = transform.translation.z;
                }
                follower.target = Some(target);
                //If just starting and close to the location match the speed.
                if transform.translation.distance(target) < 0.1 {
                    transform.translation = target;
                }
            }

            let aim_ahead_target = target + target_velocity * 5.;

            let goal_speed =
                if transform.translation.distance(aim_ahead_target) < follower.target_area {
                    if rigidbody.velocity.length() < 0.1 {
                        follower.target = None
                    };
                    0.
                } else {
                    //Aim to catch up in 5 frames clamp to max
                    ((aim_ahead_target - transform.translation) / 5.).length()
                } / time.delta_seconds();

            gizmos.circle(aim_ahead_target, Vec3::Z, 3., Color::GREEN);
            gizmos.circle(transform.translation, Vec3::Z, 3., Color::BLACK);
            let targetv = ((aim_ahead_target - transform.translation).normalize_or_zero()
                * goal_speed)
                .clamp(-follower.max_v, follower.max_v);
            let dv = (targetv - rigidbody.velocity) / time.delta_seconds();
            if dv.dot(rigidbody.velocity).is_sign_negative() && rigidbody.velocity.length() > 0.01 {
                rigidbody.drag = dv.clamp(-follower.max_d, follower.max_d);
            } else {
                //Use fraction of max_v as a proxy for whether we're just speeding up now so we don't have instant jerky acceleration;
                let new_max_a = follower.max_a;
                rigidbody.acceleration = dv.clamp(-new_max_a, new_max_a);
            };
        }
    }
}

fn follower_target_gizmo(mut gizmos: Gizmos, followers_q: Query<(&Transform, &Follower)>) {
    for (transform, follower) in followers_q.iter() {
        if let Some(target) = follower.target {
            gizmos.circle(target, Vec3::Z, 1., Color::RED);
        }
        gizmos.rect(
            transform.translation,
            Quat::from_rotation_z(0.),
            Vec2::new(follower.deadzone.x, follower.deadzone.y),
            Color::RED,
        );
    }
}
