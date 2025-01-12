use bevy::prelude::*;

use super::components::PlaneComponent;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba(0.448, 0.71, 0.87, 1.0),
                ..default()
            })),
        ))
        .insert(PlaneComponent);
}

pub fn system(mut query: Query<&mut Transform, With<PlaneComponent>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() / 4.0);
    }
}
