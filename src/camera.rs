use bevy::{camera::ScalingMode, prelude::*};

use crate::game_consts;

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

/// 相机跟随的目标
#[derive(Component, Default)]
pub struct Target;

fn setup_main_camera(mut commands: Commands) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scaling_mode = ScalingMode::AutoMin {
        min_width: game_consts::VIEWPORT_WIDTH,
        min_height: game_consts::VIEWPORT_HEIGHT,
    };

    commands.spawn((Projection::Orthographic(projection), MainCamera));
}

fn follow_target(
    target_transform: Option<Single<&Transform, With<Target>>>,
    mut camera_transform: Single<&mut Transform, (With<MainCamera>, Without<Target>)>,
) {
    let Some(target_transform) = target_transform else {
        return;
    };

    camera_transform.translation.x = target_transform.translation.x;
    camera_transform.translation.y = target_transform.translation.y;
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_main_camera)
        .add_systems(PostUpdate, follow_target);
}
