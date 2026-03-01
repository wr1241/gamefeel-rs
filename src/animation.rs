use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    pub fn fps(fps: f32) -> Self {
        Self(Timer::from_seconds(1.0 / fps, TimerMode::Repeating))
    }
}

fn update_animation(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &AnimationIndices, &mut Sprite)>,
) {
    query
        .iter_mut()
        .for_each(|(mut timer, indices, mut sprite)| {
            timer.tick(time.delta());

            if timer.just_finished()
                && let Some(atlas) = &mut sprite.texture_atlas
            {
                atlas.index = if atlas.index >= indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                }
            }
        });
}

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_animation);
}
