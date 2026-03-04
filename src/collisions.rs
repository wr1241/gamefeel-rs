mod debug;

use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
pub struct Wall;

fn wall_initial_rigid_body(_: IntGridCell) -> RigidBody {
    RigidBody::Static
}

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,

    #[with(wall_initial_rigid_body)]
    rigid: RigidBody,
}

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_int_cell::<WallBundle>(1);

    app.add_plugins(debug::plugin);
}
