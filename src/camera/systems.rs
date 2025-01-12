use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3d { ..default() },
        Transform::from_xyz(0.0, 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
