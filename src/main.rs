use bevy::prelude::*;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.1, 0.2, 0.3).into()),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Torus {
            radius: 1.0,
            ring_radius: 0.5,
            subdivisions_segments: 24,
            subdivisions_sides: 16,
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(2.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 4,
        })),
        material: materials.add(Color::rgb(0.2, 0.4, 0.3).into()),
        transform: Transform::from_xyz(-2.0, 0.5, 0.0),
        ..default()
    });
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}