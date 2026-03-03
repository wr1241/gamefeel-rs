use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkIntCell, app::LdtkIntCellAppExt};

#[derive(Component, Default)]
pub struct Wall;

#[derive(Bundle, Default, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

pub(super) fn plugin(app: &mut App) {
    app.register_ldtk_int_cell::<WallBundle>(1);
}
