use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioSource};

#[derive(Default, Clone, Debug)]
pub struct MusicState {
    pub tracks: Vec<Handle<AudioSource>>,
    pub next_track_index: Option<usize>,
    pub channel: AudioChannel,

    pub last_track_change: f64,
    pub changing_from: usize, // aka current
    pub changing_to: usize,
}

impl MusicState {
    pub fn switch_tracks(&mut self, index: usize) {
        // There is a race condition where the initial setup of the environment may get called before we've loaded our tracks.
        // We handle this by just playing the home music track when the game starts, and ignoring invalid indicies.
        if self.tracks.len() <= index {
            println!("Invalid track selection - {}", index);
            return;
        }

        self.next_track_index = Some(index);
        self.changing_to = index;
    }
}

const TRACK_CHANGE_TIME: f64 = 0.5;

pub fn setup_music(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut music_state: ResMut<MusicState>,
) {
    music_state.channel = AudioChannel::new("music".to_owned());
    music_state.tracks = vec![
        asset_server.load("music/home.mp3"),
        asset_server.load("music/park.mp3"),
        asset_server.load("music/test.mp3"),
    ];

    audio.play_looped_in_channel(music_state.tracks[0].clone(), &music_state.channel);
    audio.set_volume_in_channel(1., &music_state.channel);
    music_state.last_track_change = -20.; // hack to get the first one immediately in playing state
}

pub fn music_system(audio: Res<Audio>, mut music_state: ResMut<MusicState>, time: Res<Time>) {
    if let Some(index) = music_state.next_track_index.take() {
        music_state.last_track_change = time.seconds_since_startup();
        music_state.changing_to = index;
    }

    let time_change = time.seconds_since_startup() - music_state.last_track_change;
    if time_change < TRACK_CHANGE_TIME {
        if music_state.changing_from == music_state.changing_to {
            // Fading in (or an aborted change)
            audio.set_volume_in_channel(((time_change)/TRACK_CHANGE_TIME) as f32, &music_state.channel);
        } else {
            // Fading out
            audio.set_volume_in_channel((1. - (time_change)/TRACK_CHANGE_TIME) as f32, &music_state.channel);
        }
    } else {
        if music_state.changing_from != music_state.changing_to {
            // Actually do the change
            audio.set_volume_in_channel(0., &music_state.channel);
            music_state.changing_from = music_state.changing_to;
            println!("Switching to track {}", music_state.changing_to);
            music_state.last_track_change = time.seconds_since_startup();
            audio.stop_channel(&music_state.channel);
            audio.play_looped_in_channel(music_state.tracks[music_state.changing_to].clone(), &music_state.channel);
        } else {
            // Nothing to do
        }
    }
}
