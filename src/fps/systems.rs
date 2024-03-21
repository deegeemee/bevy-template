use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use super::components::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(
            TextBundle::from_sections([TextSection::new(
                "\nfps: ",
                TextStyle {
                    font: asset_server.load("fonts/roboto_mono_for_powerline.ttf"),
                    font_size: 20.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            )])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            }),
        )
        .insert(StatsText);
}

pub fn system(diagnostics: Res<DiagnosticsStore>, mut query: Query<&mut Text, With<StatsText>>) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[0].value = format!("{average:.2} fps");
        }
    };
}
