use bevy::prelude::*;
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

use crate::{camera::CameraPointEvent, physics::Rigidbody};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerControllerParams>()
            .register_type::<PlayerControllerParams>()
            .add_plugins(ResourceInspectorPlugin::<PlayerControllerParams>::default())
            .add_systems(Update, (moving_physics, set_camera_location))
            .add_systems(Startup, init_player);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Facing(Direction);

enum Direction {
    Left,
    Right,
    //UP,
    // DOWN,
}

#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PlayerControllerParams {
    pub jump_height: f32,
    pub jump_speed: f32,
    pub fall_speed: f32,
    pub run_max_speed: f32,
    pub run_acc: f32,
    pub run_dec: f32,
    pub camera_lookahead: f32,
}

impl Default for PlayerControllerParams {
    fn default() -> Self {
        PlayerControllerParams {
            jump_height: 10.,
            jump_speed: 10.,
            fall_speed: 10.,
            run_max_speed: 800.,
            run_acc: 400.,
            run_dec: 2000.,
            camera_lookahead: 100.,
        }
    }
}

fn init_player(mut commands: Commands) {
    // Rectangle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
        Player,
        Rigidbody::default(),
        Name::new("Player"),
        Facing(Direction::Left),
    ));
}

fn moving_physics(
    mut player_query: Query<(&mut Facing, &mut Rigidbody), With<Player>>,
    player_controller_settings: Res<PlayerControllerParams>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok((mut facing, mut rigidbody)) = player_query.get_single_mut() {
        rigidbody.max_v = Some(player_controller_settings.run_max_speed);
        if input.pressed(KeyCode::Left) {
            rigidbody.acceleration.x = -player_controller_settings.run_acc;
            *facing = Facing(Direction::Left);
        } else if input.pressed(KeyCode::Right) {
            rigidbody.acceleration.x = player_controller_settings.run_acc;
            *facing = Facing(Direction::Right);
            println!("Right")
        } else {
            rigidbody.acceleration = Vec3::ZERO;
            rigidbody.drag.x = match rigidbody.velocity.x {
                x if x > 0. => -player_controller_settings.run_dec,
                x if x < 0. => player_controller_settings.run_dec,
                _ => 0.,
            }
        }
    }
}

//Maybe replace facing with animation state
#[allow(clippy::type_complexity)]
fn set_camera_location(
    player_q: Query<
        (&Transform, &Facing, &Rigidbody),
        (With<Player>, Or<(Changed<Transform>, Changed<Facing>)>),
    >,
    mut send_camera_location: EventWriter<CameraPointEvent>,
    settings: Res<PlayerControllerParams>,
) {
    if let Ok((transform, facing, rigidbody)) = player_q.get_single() {
        let lookahead = match facing.0 {
            Direction::Left => Vec3::NEG_X * settings.camera_lookahead,
            Direction::Right => Vec3::X * settings.camera_lookahead,
        }; //* (rigidbody.velocity.length() / settings.run_max_speed).abs();
        send_camera_location.send(CameraPointEvent {
            location: Some(transform.translation + lookahead),
            zoom: None,
            pan_speed: Some((rigidbody.velocity.length() + 100.) * Vec3::ONE),
        })
    }
}
