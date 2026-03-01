use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::camera;

#[derive(Component, Default)]
#[require(camera::Target)]
pub struct Player;

#[derive(Bundle, Default, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
}

pub fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("PlayerStart");
}
