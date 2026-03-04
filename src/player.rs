mod debug;
mod input;

use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::{AnimationBundle, AnimationIndices, AnimationTimer},
    camera,
    physics::Mass,
};

#[derive(Component, Debug, Default, Eq, PartialEq, Reflect)]
enum PlayerState {
    #[default]
    Idle,
    IdleWeapon,
    JumpDown,
    JumpUp,
    Land,
    Run,
    ClimbStep,
    RunWeapon,
    Shoot,
    ReadyWeapon,
}

impl PlayerState {
    fn get_animation_timer(&self) -> AnimationTimer {
        match self {
            _ => AnimationTimer::fps(8.0),
        }
    }

    fn get_animation_indices(&self) -> AnimationIndices {
        match self {
            Self::Idle => AnimationIndices { first: 0, last: 7 },
            Self::IdleWeapon => AnimationIndices { first: 8, last: 8 },
            Self::JumpDown => AnimationIndices {
                first: 16,
                last: 16,
            },
            Self::JumpUp => AnimationIndices {
                first: 24,
                last: 24,
            },
            Self::Land => AnimationIndices {
                first: 32,
                last: 34,
            },
            Self::Run => AnimationIndices {
                first: 40,
                last: 43,
            },
            Self::ClimbStep => AnimationIndices {
                first: 48,
                last: 49,
            },
            Self::RunWeapon => AnimationIndices {
                first: 56,
                last: 59,
            },
            Self::Shoot => AnimationIndices {
                first: 64,
                last: 66,
            },
            Self::ReadyWeapon => AnimationIndices {
                first: 72,
                last: 79,
            },
        }
    }
}

impl From<&EntityInstance> for PlayerState {
    fn from(_value: &EntityInstance) -> Self {
        Self::Idle
    }
}

fn player_initial_animation(_: &EntityInstance) -> AnimationBundle {
    AnimationBundle {
        indices: PlayerState::default().get_animation_indices(),
        timer: PlayerState::default().get_animation_timer(),
    }
}

// fn player_initial_mass(_: &EntityInstance) -> Mass {
//     Mass(1.0)
// }

fn player_initial_rigid_body(_: &EntityInstance) -> RigidBody {
    RigidBody::Kinematic
}

#[derive(Component, Default)]
#[require(camera::Target)]
#[component(on_add = input::setup_player_input)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[default]
    player: Player,

    #[from_entity_instance]
    player_state: PlayerState,

    #[sprite_sheet("hero.png", 64, 64, 8, 10, 0, 0, 0)]
    sprite: Sprite,

    #[with(player_initial_animation)]
    animation: AnimationBundle,

    // #[with(player_initial_mass)]
    // mass: Mass,
    #[with(player_initial_rigid_body)]
    rigid_body: RigidBody,
}

fn update_player_animation(
    query: Single<(&mut AnimationTimer, &mut AnimationIndices, &PlayerState), Changed<PlayerState>>,
) {
    let (mut timer, mut indices, state) = query.into_inner();

    *timer = state.get_animation_timer();
    *indices = state.get_animation_indices();
}

pub fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("PlayerStart");
    app.register_type::<PlayerState>();

    app.add_plugins(input::plugin).add_plugins(debug::plugin);

    app.add_systems(PostUpdate, update_player_animation);
}
