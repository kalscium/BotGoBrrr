use bevy::{input::gamepad::{GamepadConnection, GamepadEvent}, prelude::*, window::PrimaryWindow};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .add_systems(Startup, (spawn_camera, spawn_robot, spawn_text))
        .add_systems(PreUpdate, gamepad_connections)
        .add_systems(FixedUpdate, (keyboard_movement, gamepad_movement))
        .add_systems(PostUpdate, confine_movement)
        .run();
}

#[derive(Component)]
pub struct Robot {
    movement_speed: f32,
    rotation_speed: f32,
}

/// Simple resource to store the ID of the first connected gamepad.
/// We can use it to know which gamepad to use for player input.
#[derive(Resource)]
pub struct MyGamepad(Gamepad);

fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut evr_gamepad: EventReader<GamepadEvent>,
) {
    for ev in evr_gamepad.read() {
        // we only care about connection events
        let GamepadEvent::Connection(ev_conn) = ev else {
            continue;
        };
        match &ev_conn.connection {
            GamepadConnection::Connected(info) => {
                debug!(
                    "New gamepad connected: {:?}, name: {}",
                    ev_conn.gamepad, info.name,
                );
                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(ev_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                debug!("Lost connection with gamepad: {:?}", ev_conn.gamepad);
                // if it's the one we previously used for the player, remove it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == ev_conn.gamepad {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
        }
    }
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
            rotation_speed: 4.0,
        },
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
        transform: Transform::from_xyz(window.width() / 2.0, window.height(), 2.0), // make the camera above everything
        ..Default::default()
    });
}

fn get_text_style(asset_server: Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("JetBrainsMono-Medium.ttf"),
        font_size: 64.0,
        ..Default::default()
    }
}

pub fn spawn_text(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.single();

    let text_style = get_text_style(asset_server);

    commands.spawn(Text2dBundle {
        text: Text::from_section("hello\nworld", text_style)
            .with_justify(JustifyText::Left),
        transform: Transform::from_xyz(window.width() / 2.0, window.height(), 1.0), // make the text in the middle of the screen
        ..Default::default()
    });
}

pub fn gamepad_movement(
    time: Res<Time>,
    axes: Res<Axis<GamepadAxis>>,
    gamepad: Option<Res<MyGamepad>>,
    mut robot_query: Query<(&Robot, &mut Transform)>,
    mut text_query: Query<&mut Text>,
    asset_server: Res<AssetServer>,
) {
    let mut text = text_query.single_mut();
    let (robot, mut transform) = robot_query.single_mut();
    let Some(&MyGamepad(gamepad)) = gamepad.as_deref() else {
        // no gamepad is connected so do nothing
        return;
    };

    // joystick getters
    let axis_jx = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickY,
    };
    let axis_jy = GamepadAxis {
        gamepad, axis_type: GamepadAxisType::LeftStickX,
    };

    // get the joystick x and y values
    let (jx, jy) = (-axes.get(axis_jx).unwrap(), -axes.get(axis_jy).unwrap());

    // get the left and right drive values
    let (ldr, rdr, debug_info) = crate::controls::controls(jx, jy, transform.rotation.z * -180.0);

    // update text
    let debug_info = debug_info.join("\n");
    *text = Text::from_section(debug_info, get_text_style(asset_server));

    // convert them into an f32 `-1..=1`
    let ldr = ldr as f32 / 12000.0;
    let rdr = rdr as f32 / 12000.0;

    // get the rotation and movement factors
    let rotation_factor = rdr - ldr;
    let movement_factor = ldr + rdr;

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
