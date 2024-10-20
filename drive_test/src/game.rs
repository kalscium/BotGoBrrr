use bevy::{prelude::*, window::PrimaryWindow};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, (spawn_camera, spawn_robot))
        .add_systems(FixedUpdate, (keyboard_movement, confine_movement))
        .run();
}

#[derive(Component)]
pub struct Robot {
    movement_speed: f32,
    rotation_speed: f32,
}

#[derive(Component)]
pub struct DriveTrain {
    ldr: i32,
    rdr: i32,
}

pub fn spawn_robot(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Robot {
            movement_speed: 512.0,
            rotation_speed: 2.4
        },
        DriveTrain { ldr: 0, rdr: 0 },
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height(), 0.0) // make the robot in the middle of the screen
                .with_scale(Vec3::splat(0.04)), // scale down the robot
            texture: asset_server.load("robot.jpg"), // get the robot texture
            ..Default::default()
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height(), 1.0), // make the camera above everything
        ..Default::default()
    });
}

pub fn keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Robot, &mut Transform)>,
) {
    let (robot, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        rotation_factor -= 1.0;
    }

    // update the robot rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * robot.rotation_speed * time.delta_seconds());

    // get the ship's forward vector by applying the current rotation to the robot's initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the robot will move based on direction, the robot's movement speed and delta time
    let movement_distance = movement_factor * robot.movement_speed * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation data
    transform.translation += translation_delta;
}

pub fn confine_movement(
    mut robot_query: Query<&mut Transform, With<Robot>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let transform = &mut robot_query.get_single_mut().unwrap();
    let window = window_query.get_single().unwrap();

    let translation = &mut transform.translation;

    // clamp the x and y position
    translation.x = translation.x.clamp(0.0, window.width() - window.width() / 16.0);
    translation.y = translation.y.clamp(window.height() / 16.0, window.height());
}
