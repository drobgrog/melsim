use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::{game,npc};

const COVID_RISK_MULTIPLIER: f32 = 0.4;
const COVID_SAFETY_DISTANCE: f32 = 6.;

pub fn covid_system(
    covid_info: Query<(&npc::NPC, &RigidBodyPositionComponent)>,
    player_info: Query<(&Player, &RigidBodyPositionComponent)>,
    mut state: ResMut<game::GameState>,
    time: Res<Time>,
) {
    let (_, player_pos) = player_info.single();
    let player_vector = player_pos.position.translation.vector;

    let mut covid_risk = 0.;

    for (_, position) in covid_info.iter() {
        let person_vector = position.position.translation.vector;
        let displacement = player_vector - person_vector;
        let d = displacement.magnitude();
        if d < COVID_SAFETY_DISTANCE {
            covid_risk += COVID_RISK_MULTIPLIER * (COVID_SAFETY_DISTANCE - d);
        }
    }

    state.set_covid_risk(covid_risk, &time);
}
