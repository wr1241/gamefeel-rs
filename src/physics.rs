use bevy::prelude::*;

const DEFAULT_GRAVITY: f32 = 9.8;

#[derive(Resource)]
pub struct Gravity(pub f32);

#[derive(Component)]
pub struct Mass(pub f32);

fn apply_gravity(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut bodies: Query<(&mut Transform, &Mass)>,
) {
    bodies.iter_mut().for_each(|(mut transform, mass)| {
        if mass.0 > 0.0 {
            transform.translation.y -= gravity.0 * time.delta_secs().powi(2) * 300.0;
        }
    });
}

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gravity(DEFAULT_GRAVITY));
    app.add_systems(
        PostUpdate,
        apply_gravity.before(TransformSystems::Propagate),
    );
}
