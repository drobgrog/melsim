use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;

#[derive(Debug, Clone, Component)]
pub struct Person {}

pub fn covid_system(
    covid_info: Query<(&Person, &RigidBodyPositionComponent)>,
    player_info: Query<(&Player, &RigidBodyPositionComponent)>,
) {
    let (_, player_pos) = player_info.single();
    let player_vector = player_pos.position.translation.vector;

    for (_, position) in covid_info.iter() {
        let person_vector = position.position.translation.vector;
        let distance = player_vector - person_vector;
        println!("Player is {:?} away", distance);
    }
}
