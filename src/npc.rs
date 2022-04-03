use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct NPC {}

impl NPC {
    fn new() -> Self {
        Self {}
    }
}

use crate::{
    player::{Player, SPRITE_SIZE_X, SPRITE_SIZE_Y},
    TILE_SIZE,
};

pub fn npc_system(mut player_info: Query<(&Player, &mut RigidBodyVelocityComponent)>) {}

pub fn spawn_npc(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    println!(
        "Spawning NPC: size: x: {:?} y: {:?}",
        SPRITE_SIZE_X, SPRITE_SIZE_Y
    );

    // Set the size of the collider
    let collider_size_x = (SPRITE_SIZE_X / TILE_SIZE) / 2.;
    let collider_size_y = (SPRITE_SIZE_Y / TILE_SIZE) / 2.;

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
            body_type: RigidBodyType::KinematicVelocityBased.into(),
            mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            position: [0., -5.].into(),
            shape: ColliderShape::cuboid(collider_size_x, collider_size_y).into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1));
}
