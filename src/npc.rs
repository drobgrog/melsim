use bevy::prelude::*;
use bevy_rapier2d::{
    na::{Isometry2, Vector2},
    prelude::*,
};
use rand::prelude::*;

#[derive(Component)]
pub struct NPC {
    pub last_moved: Timer,
    pub velocity: Vector2<f32>,
}

impl NPC {
    fn new() -> Self {
        Self {
            last_moved: Timer::from_seconds(1.0, true),
            velocity: [1.0, 0.0].into(),
        }
    }
}

use crate::{
    environment::tile_coords_to_screen_pos,
    player::{SPRITE_SIZE_X, SPRITE_SIZE_Y},
    TILE_SIZE,
};

pub fn npc_system(
    mut npc_query: Query<(&mut NPC, &mut RigidBodyVelocityComponent)>,
    time: Res<Time>,
) {
    for (mut npc, mut rigid_body_velocity) in npc_query.iter_mut() {
        {
            let timer = &mut npc.last_moved;
            timer.tick(time.delta());
        }

        let timer = &npc.last_moved;
        if timer.just_finished() {
            let mut rng = rand::thread_rng();
            let rand: f64 = rng.gen();
            if rand >= 0.80 {
                set_new_direction(rand, &mut npc.velocity);
            }
        }

        rigid_body_velocity.linvel = npc.velocity;

        // TODO: If NPC collides with wall, IMMEDIATELY change position.
    }
}

fn set_new_direction(rand: f64, current_velocity: &mut Vector2<f32>) {
    let theta = if rand <= 0.85 {
        std::f32::consts::PI / 2.
    } else if rand <= 0.90 {
        std::f32::consts::PI
    } else {
        std::f32::consts::PI + std::f32::consts::PI / 2.
    };

    let iso = Isometry2::rotation(theta);
    *current_velocity = iso.transform_vector(current_velocity);

    println!(
        "UPDATED VELOCITY: {:?} - THETA: {:?}",
        current_velocity, theta
    );
}

// TODO: add ability to set location of NPC
pub fn spawn_npc(commands: &mut Commands, asset_server: &Res<AssetServer>, position: [usize; 2]) {
    println!(
        "Spawning NPC: size: x: {:?} y: {:?}",
        SPRITE_SIZE_X, SPRITE_SIZE_Y
    );

    // Set the size of the collider
    let collider_size_x = (SPRITE_SIZE_X / TILE_SIZE) / 2.;
    let collider_size_y = (SPRITE_SIZE_Y / TILE_SIZE) / 2.;

    // Get the position
    let (x_pos, y_pos) = (position[0], position[1]);
    let (pos_x, pos_y) = tile_coords_to_screen_pos(x_pos, 2., y_pos, 3.);

    println!(
        "Collider size: x: {:?} y: {:?}",
        collider_size_x, collider_size_y
    );

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("npc.png"),
            transform: Transform {
                translation: [0., 0., 1.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NPC::new())
        .insert_bundle(RigidBodyBundle {
            position: [pos_x / TILE_SIZE, pos_y / TILE_SIZE].into(),
            velocity: RigidBodyVelocity::new([1., 0.].into(), Default::default()).into(),
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_x, collider_size_y).into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1));
}
