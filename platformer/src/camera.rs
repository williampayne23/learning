use crate::{follow::Follower, physics::Rigidbody};
use bevy::prelude::*;
pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(Update, set_location)
            .add_event::<CameraPointEvent>();
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Event, Default)]
pub struct CameraPointEvent {
    pub location: Option<Vec3>,
    pub zoom: Option<Vec3>,
    pub pan_speed: Option<Vec3>,
}

fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
        Follower {
            target: None,
            max_v: Vec3::ONE * 800.,
            max_d: Vec3::ONE * 2000.,
            max_a: Vec3::ONE * 400.,
            target_area: 2.,
            deadzone: Vec3::new(0., 0., 100.),
            ..default()
        },
        Rigidbody::from_max_v(10000.),
    ));
}

fn set_location(
    mut camera_follower_q: Query<(&mut Follower, &Transform), With<MainCamera>>,
    mut point_events: EventReader<CameraPointEvent>,
) {
    for ev in point_events.iter() {
        for (mut follower, _transform) in camera_follower_q.iter_mut()
        {
            if let Some(target) = ev.location {
                if _transform.translation.distance(target) > 0.1 {
                    follower.target = Some(target);
                }
            }
            if let Some(pan_speed) = ev.pan_speed {
                follower.max_v = pan_speed;
            }
        }
    }
}
