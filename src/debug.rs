use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

#[derive(Component)]
struct DebugInfo;

#[derive(Component)]
struct EntityCount(usize);

fn display_debug_info(mut commands: Commands) {
    commands.spawn((
        DebugInfo,
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

fn update_debug_info(
    entity_count: Single<&EntityCount>,
    mut text: Single<&mut Text, With<DebugInfo>>,
) {
    text.0 = format!("Entity Count: {}", entity_count.0);
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default());

    app.add_systems(Startup, display_debug_info)
        .add_systems(Update, update_debug_info)
        .add_systems(Update, update_entity_count.before(update_debug_info));
}
