use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(Component, Default)]
pub struct DebugInfo(pub String);

#[derive(Component)]
struct EntityCount(usize);

fn display_debug_info(mut commands: Commands) {
    commands.spawn((
        DebugInfo::default(),
        EntityCount(0),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..Default::default()
        },
        Text::new("DEBUG INFO"),
        TextLayout::new_with_justify(Justify::Right),
        TextColor(Color::WHITE),
    ));
}

fn update_entity_count(
    all_entities: Query<Entity>,
    mut entity_count: Option<Single<&mut EntityCount>>,
) {
    if let Some(entity_count) = entity_count.as_mut() {
        entity_count.0 = all_entities.count();
    }
}

fn update_debug_info(entity_count: Single<&EntityCount>, debug: Single<(&mut Text, &DebugInfo)>) {
    let (mut text, debug_info) = debug.into_inner();
    text.0 = format!("Entity Count: {}\n{}", entity_count.0, debug_info.0);
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default());

    app.add_systems(Startup, display_debug_info)
        .add_systems(Update, update_debug_info)
        .add_systems(Update, update_entity_count.before(update_debug_info));
}
