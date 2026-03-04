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
    player: Option<Single<(&mut Transform, &mut super::PlayerState, &mut Sprite), With<OnGround>>>,
) {
    let Some(player) = player else {
        return;
    };

    let (mut transform, mut state, mut sprite) = player.into_inner();

    if movement.x != 0.0 {
        transform.translation.x += movement.x * 200.0 * time.delta_secs();
        if *state != super::PlayerState::Run {
            *state = super::PlayerState::Run;
        }

        if movement.x == 1.0 {
            sprite.flip_x = false;
        } else if movement.x == -1.0 {
            sprite.flip_x = true;
        }
    } else {
        if *state != super::PlayerState::Idle {
            *state = super::PlayerState::Idle;
        }
    }

    // debug purpose
    if movement.y != 0.0 {
        transform.translation.y += movement.y * 200.0 * time.delta_secs();
        if *state != super::PlayerState::Run {
            *state = super::PlayerState::Run;
        }
    } else {
        if *state != super::PlayerState::Idle {
            *state = super::PlayerState::Idle;
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_input_context::<OnGround>();
    app.add_systems(FixedUpdate, apply_movement);
}
