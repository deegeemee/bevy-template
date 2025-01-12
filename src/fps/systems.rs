use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::components::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Text::new(String::from("")),
            TextFont {
                font: asset_server.load("fonts/roboto_mono_for_powerline.ttf"),
                font_size: 16.0,
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..default()
            },
        ))
        .insert(StatsText);
}

pub fn system(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<Entity, With<StatsText>>,
    mut writer: TextUiWriter,
) {
    let entity = query.single();
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            *writer.text(entity, 0) = format!("{average:.2} fps");
        }
    };
}
