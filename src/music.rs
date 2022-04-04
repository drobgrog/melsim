use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioSource};

#[derive(Default, Clone, Debug)]
pub struct MusicState {
    pub tracks: Vec<Handle<AudioSource>>,
    pub current_track_index: usize,
    pub next_track_index: Option<usize>,
    pub channel: AudioChannel,
}

impl MusicState {
    pub fn switch_tracks(&mut self, index: usize) {
        if self.tracks.len() <= index {
            panic!("Invalid track selection - {}", index);
        }

        self.next_track_index = Some(index);
    }
}

pub fn setup_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut music_state: ResMut<MusicState>,
) {
    music_state.channel = AudioChannel::new("music".to_owned());
    music_state.tracks = vec![
        asset_server.load("music/test.mp3"),
        asset_server.load("music/test_2.mp3"),
    ];

    audio.play_looped_in_channel(music_state.tracks[0].clone(), &music_state.channel);
}

pub fn music_system(audio: Res<Audio>, mut music_state: ResMut<MusicState>) {
    if let Some(index) = music_state.next_track_index.take() {
        println!("Switching to track {}", index);
        audio.stop_channel(&music_state.channel);
        audio.play_looped_in_channel(music_state.tracks[index].clone(), &music_state.channel);
        music_state.current_track_index = index;
    }
}
