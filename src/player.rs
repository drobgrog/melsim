use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::na::Vector2;

use crate::TILE_SIZE;

pub static SPRITE_SIZE_X: f32 = 100.0;
pub static SPRITE_SIZE_Y: f32 = 150.0;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

pub fn player_movement(
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

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.speed;
    }
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Sprite size: x: {:?} y: {:?}", SPRITE_SIZE_X, SPRITE_SIZE_Y);

    // Set the size of the collider
    let collider_size_x = (SPRITE_SIZE_X / TILE_SIZE) / 2.;
    let collider_size_y = (SPRITE_SIZE_Y / TILE_SIZE) / 2.;

    println!(
        "Collider size: x: {:?} y: {:?}",
        collider_size_x, collider_size_y
    );

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform {
                translation: [0., 0., 1.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 300.0 })
        .insert_bundle(RigidBodyBundle {
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            position: [0., 0.].into(),
            shape: ColliderShape::cuboid(collider_size_x, collider_size_y).into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1));
}
