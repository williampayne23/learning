use bevy::{prelude::*, window::PrimaryWindow};

pub struct WorldToScreenPLugin {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct WorldPos {
    pub translation: Vec2,
    pub rotation: Quat,
    pub scale: Vec2,
}

impl WorldPos {
    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec2::new(1.0, 1.0),
        }
    }

    pub fn from_screen_pos(screen_pos: &Vec2, world_size: &Vec2, screen_size: &Vec2) -> Self {
        let translation = convert_screen_to_world_pos(screen_pos, world_size, screen_size);
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec2::new(1.0, 1.0),
        }
    }
}

#[derive(Resource)]
pub struct WorldToScreenConfig {
    pub width: f32,
    pub height: f32,
}

impl Plugin for WorldToScreenPLugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldToScreenConfig {
            width: self.width,
            height: self.height,
        })
        .add_systems(PostUpdate, world_to_screen)
        .add_observer(ensure_transform_with_worldpos);
    }
}

fn ensure_transform_with_worldpos(trigger: Trigger<OnAdd, WorldPos>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)));
}

fn world_to_screen(
    mut query: Query<(&mut Transform, &WorldPos)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    world_to_screen_config: Res<WorldToScreenConfig>,
) {
    let primary_window = window_query.single().expect("No primary window found");
    let window_size = primary_window.resolution.physical_size().as_vec2();
    let window_scale = primary_window.scale_factor();
    let logical_size = Vec2::new(
        window_size.x as f32 / window_scale,
        window_size.y as f32 / window_scale,
    );
    let world_size = Vec2::new(world_to_screen_config.width, world_to_screen_config.height);

    let proportional_screen_size = get_proportional_size(&world_size, &logical_size);

    for (mut transform, world_pos) in query.iter_mut() {
        let new_pos = convert_world_to_screen_pos(
            &world_pos.translation,
            &world_size,
            &proportional_screen_size,
        );
        // Update the transform with the new screen position
        transform.translation = Vec3::new(new_pos.x, new_pos.y, transform.translation.z);

        // Apply rotation and scale
        transform.rotation = world_pos.rotation;

        let screen_scale_x = proportional_screen_size.x / world_size.x;
        let screen_scale_y = proportional_screen_size.y / world_size.y;
        // Scale the transform based on the world position scale
        transform.scale = Vec3::new(
            world_pos.scale.x * screen_scale_x,
            world_pos.scale.y * screen_scale_y,
            1.0,
        );
    }
}

pub fn get_proportional_size(world_size: &Vec2, logical_size: &Vec2) -> Vec2 {
    // Larges possible size that fits in the logical size
    // But is the same aspect ratio as the world size
    let scale_x = logical_size.x / world_size.x;
    let scale_y = logical_size.y / world_size.y;
    let scale = scale_x.min(scale_y);
    Vec2::new(world_size.x * scale, world_size.y * scale)
}

pub fn convert_world_to_screen_pos(
    world_pos: &Vec2,
    world_size: &Vec2,
    screen_size: &Vec2,
) -> Vec2 {
    let proportional_screen_size = get_proportional_size(world_size, screen_size);
    let screen_x = (world_pos.x - world_size.x / 2.) * proportional_screen_size.x / world_size.x;
    let screen_y = (world_pos.y - world_size.y / 2.) * proportional_screen_size.y / world_size.y;
    Vec2::new(screen_x, screen_y)
}

pub fn convert_screen_to_world_pos(
    screen_pos: &Vec2,
    world_size: &Vec2,
    screen_size: &Vec2,
) -> Vec2 {
    let proportional_screen_size = get_proportional_size(world_size, screen_size);
    let world_x = screen_pos.x * world_size.x / proportional_screen_size.x + world_size.x / 2.;
    let world_y = screen_pos.y * world_size.y / proportional_screen_size.y + world_size.y / 2.;
    Vec2::new(world_x, world_y)
}
