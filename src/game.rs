use bevy::prelude::*;

#[derive(Default)]
pub struct GameState {
    messages: Vec<TextMessage>,
    pub date: i32,
}

struct TextMessage {
    text: String,
}

pub fn logic(mut state: ResMut<GameState>, time: Res<Time>) {
    state.date = 1 + (time.seconds_since_startup() / 5.) as i32;
}
