use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            radius: 0.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(2.5, 8.0, 2.5),
    ));
}
