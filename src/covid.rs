use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::Player;
use crate::{game,npc,environment,music};

const COVID_RISK_MULTIPLIER: f32 = 0.4;
const COVID_SAFETY_DISTANCE: f32 = 6.;

pub fn covid_system(
    covid_info: Query<(&npc::NPC, &RigidBodyPositionComponent)>,
    mut player_info: Query<(&Player, &mut RigidBodyPositionComponent), Without<npc::NPC>>,
    mut state: ResMut<game::GameState>,
    time: Res<Time>,
    mut environment_query: Query<(&mut TextureAtlasSprite, &mut environment::Environment)>,
    environment_collider_query: Query<Entity, With<environment::EnvironmentCollider>>,
    mut music_state: ResMut<music::MusicState>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let (_, mut player_pos) = player_info.single_mut();
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
    if covid_risk >= 1. {
        state.covid_narrative_switch(
            &time,
            &mut player_pos,
            &mut environment_query,
            &mut commands,
            &environment_collider_query,
            &mut music_state,
            &asset_server,
        );
    }
}
