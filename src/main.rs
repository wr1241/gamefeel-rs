mod animation;
mod camera;
mod collisions;
mod debug;
mod game_consts;
mod player;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_enhanced_input::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(EnhancedInputPlugin)
        .add_plugins(animation::plugin)
        .add_plugins(camera::plugin)
        .add_plugins(collisions::plugin)
        .add_plugins(debug::plugin)
        .add_plugins(player::plugin)
        .add_systems(Startup, load_level)
        .insert_resource(LevelSelection::indices(0, 0))
        .run();
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("gameFeel.ldtk").into(),
        ..Default::default()
    });
}
