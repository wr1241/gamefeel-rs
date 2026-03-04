use std::collections::HashMap;

use bevy::{input::common_conditions::input_pressed, prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::{
    GridCoords, LayerMetadata,
    utils::{grid_coords_to_ldtk_grid_coords, translation_to_grid_coords},
};

use crate::{camera::MainCamera, debug::DebugInfo};

fn show_collision_debug_info(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    tiles: Query<(Entity, &GridCoords, &ChildOf)>,
    layers: Query<&LayerMetadata>,
    mut debug_info: Single<&mut DebugInfo>,
) {
    let (camera, camera_transform) = *camera;

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {
        let mut tiles_under_cursor = HashMap::new();

        tiles
            .iter()
            .for_each(|(tile_entity, tile_grid_coords, parent_entity)| {
                if let Ok(layer) = layers.get(parent_entity.0) {
                    let grid_coords =
                        translation_to_grid_coords(pos, IVec2::splat(layer.grid_size));
                    if tile_grid_coords.eq(&grid_coords) {
                        let ldtk_coords = grid_coords_to_ldtk_grid_coords(grid_coords, layer.c_hei);

                        tiles_under_cursor
                            .entry((grid_coords, ldtk_coords, ldtk_coords * layer.grid_size))
                            .and_modify(|tile_entities: &mut Vec<Entity>| {
                                tile_entities.push(tile_entity)
                            })
                            .or_insert(vec![tile_entity]);
                    }
                }
            });

        if !tiles_under_cursor.is_empty() {
            let mut debug_str = String::new();
            tiles_under_cursor.iter().for_each(
                |((grid_coords, ldtk_coords, px_coords), tile_entities)| {
                    debug_str.push_str(&format!(
                        "Tiles ({}) at grid({},{}) ldtk({},{}) px({},{})",
                        tile_entities
                            .iter()
                            .map(|entity| entity.to_string())
                            .collect::<Vec<_>>()
                            .join(","),
                        grid_coords.x,
                        grid_coords.y,
                        ldtk_coords.x,
                        ldtk_coords.y,
                        px_coords.x,
                        px_coords.y,
                    ));
                    debug_str.push('\n');
                },
            );
            debug_info.0 = debug_str;
        } else {
            debug_info.0 = String::default();
        }
    };
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        show_collision_debug_info.run_if(input_pressed(KeyCode::ShiftLeft)),
    );
}
