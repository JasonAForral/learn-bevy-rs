use bevy::{
    prelude::*,
    time::FixedTimestep,
    sprite::MaterialMesh2dBundle
};

const TIME_STEP: f32 = 1.0 / 60.0;
const BOUNDS: Vec2 = Vec2::new(1200.0, 640.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(player_movement_system)
        )
        .add_system(bevy::window::close_on_esc)
        .run();
}

/// player component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    movement_speed: f32,
    /// rotation speed in radians per second
    rotation_speed: f32,
}   

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(Camera2dBundle::default());


    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 3).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RgbaLinear { red: 0.1, green: 0.3, blue: 0.2, alpha: 1.0 })),
            transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
            ..default()
        })
        .insert(Player {
            movement_speed: 500.0,
            rotation_speed: f32::to_radians(360.0),
        });
}

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player, &mut Transform)>
) {
    let (ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        rotation_factor += 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        movement_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement_factor -= 0.5;
    }

    transform.rotate_z(rotation_factor * ship.rotation_speed * TIME_STEP);

    let movement_direction = transform.rotation * Vec3::Y;
    let movement_distance = movement_factor * ship.movement_speed * TIME_STEP;
    let mut translation_delta = movement_direction * movement_distance;
    
    // check for wall reflection
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    let translated_transform = transform.translation + translation_delta;

    // check left and right walls
    if translated_transform.x < -extents.x || translated_transform.x > extents.x {
        transform.rotation = transform.rotation.inverse();
        translation_delta.x = -translation_delta.x;
    }

    // check top  and bottom walls
    if translated_transform.y < -extents.y || translated_transform.y > extents.y {
        let (.., z) = transform.rotation.to_euler(EulerRot::YXZ);
        transform.rotation = Quat::from_rotation_z(f32::to_radians(180.0) - z);
        translation_delta.y = -translation_delta.y;
    }

    transform.translation += translation_delta;

}