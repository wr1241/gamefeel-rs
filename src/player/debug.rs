use bevy::prelude::*;

fn show_sprite_boundary(
    mut commands: Commands,
    player: Option<Single<Entity, (Added<super::Player>, With<super::Player>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(player) = player {
        let mesh = meshes.add(Rectangle::from_size(Vec2::splat(64.0)).to_ring(1.0));
        let material = materials.add(ColorMaterial::from_color(Color::srgb(1.0, 0.0, 0.0)));

        commands
            .entity(player.into_inner())
            .insert((Mesh2d(mesh), MeshMaterial2d(material)));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, show_sprite_boundary);
}
