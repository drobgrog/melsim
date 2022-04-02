use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

#[derive(Component)]
struct Player {
    speed: f32,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Melbourne Lockdown Simulator"),
            width: 1324.,
            height: 1024.,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(255.0, 255.0, 255.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .add_system(player_movement)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("environment.png"),
        transform: Transform {
            translation: [-162., 0., 0.0].into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let sprite_size_x = 128.0;
    let sprite_size_y = 192.0;

    // Set the scale
    rapier_config.scale = 64.0;

    // Set gravity
    rapier_config.gravity = [0., 0.].into();

    // Set the size of the collider
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 300.0 })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle(ColliderBundle {
            position: [collider_size_x / 2.0, collider_size_y / 2.0].into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut player_info: Query<(&Player, &mut RigidBodyVelocityComponent)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta = Vector2::new(x_axis as f32, y_axis as f32);
        if move_delta != Vector2::zeros() {
            // Note that the RapierConfiguration::Scale factor is also used here to transform
            // the move_delta from: 'pixels/second' to 'physics_units/second'
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        println!("Move delta: {:?}", move_delta);

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.speed;
    }
}
