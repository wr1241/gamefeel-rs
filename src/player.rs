mod input;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    camera,
};

#[derive(Component, Debug, Default, Reflect)]
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
            Self::Idle => AnimationTimer::fps(8.0),
            _ => todo!(),
        }
    }

    fn get_animation_indices(&self) -> AnimationIndices {
        match self {
            Self::Idle => AnimationIndices { first: 0, last: 7 },
            _ => todo!(),
        }
    }

    fn get_texture_atlas_layout(&self) -> TextureAtlasLayout {
        match self {
            Self::Idle => TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 1, None, None),
            _ => todo!(),
        }
    }
}

impl From<&EntityInstance> for PlayerState {
    fn from(_value: &EntityInstance) -> Self {
        Self::Idle
    }
}

fn player_initial_animation_indices(_: &EntityInstance) -> AnimationIndices {
    PlayerState::default().get_animation_indices()
}

fn player_initial_animation_timer(_: &EntityInstance) -> AnimationTimer {
    PlayerState::default().get_animation_timer()
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

    #[sprite("hero.png")]
    sprite: Sprite,

    #[with(player_initial_animation_indices)]
    animation_indices: AnimationIndices,

    #[with(player_initial_animation_timer)]
    animation_timer: AnimationTimer,
}

fn update_player_animation(
    query: Single<
        (
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut Sprite,
            &PlayerState,
        ),
        Changed<PlayerState>,
    >,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let (mut timer, mut indices, mut sprite, state) = query.into_inner();

    *timer = state.get_animation_timer();
    *indices = state.get_animation_indices();

    sprite.texture_atlas = Some(TextureAtlas {
        layout: layouts.add(state.get_texture_atlas_layout()),
        index: indices.first,
    });

    sprite.custom_size = Some(Vec2::splat(64.0));
}

pub fn plugin(app: &mut App) {
    app.register_ldtk_entity::<PlayerBundle>("PlayerStart");
    app.register_type::<PlayerState>();

    app.add_plugins(input::plugin);

    app.add_systems(PreUpdate, update_player_animation);
}
