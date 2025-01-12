use bevy::prelude::*;

use super::components::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.78, 0.07, 0.51, 1.0),
                ..default()
            })),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ))
        .insert(Cube);
}

pub fn system(mut query: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 1.2);
    }
}
