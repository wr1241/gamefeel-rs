use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;

#[derive(Component)]
struct OnGround;

#[derive(InputAction)]
#[action_output(Vec2)]
struct Move;

#[derive(InputAction)]
#[action_output(bool)]
struct Jump;

pub(super) fn setup_player_input(mut world: DeferredWorld, context: HookContext) {
    world.commands().entity(context.entity).insert((
        OnGround,
        (actions!(
            OnGround[
                (
                    Action::<Move>::new(),
                    Bindings::spawn((
                        // Bindings like WASD or sticks are very common,
                        // so we provide built-in `SpawnableList`s to assign all keys/axes at once.
                        Cardinal::wasd_keys(),
                    )),
                ),
                (
                    Action::<Jump>::new(),
                    bindings![KeyCode::Space],
                )
            ]
        )),
    ));
}

fn apply_movement(
    time: Res<Time>,
    movement: Single<&Action<Move>>,
    mut transform: Option<Single<&mut Transform, With<OnGround>>>,
) {
    let Some(transform) = &mut transform else {
        return;
    };

    transform.translation.x += movement.x * 200.0 * time.delta_secs();
}

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<OnGround>();
    app.add_systems(FixedUpdate, apply_movement);
}
