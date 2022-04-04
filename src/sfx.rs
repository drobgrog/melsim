use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioSource};

#[derive(Default, Clone, Debug)]
pub struct SFXSystem {
    pub text: Handle<AudioSource>,
    pub sanity_up: Handle<AudioSource>,
    pub sanity_down: Handle<AudioSource>,
    pub pickup: Handle<AudioSource>,
    pub entrance_exit: Handle<AudioSource>,
    pub cash_register: Handle<AudioSource>,
    pub channel: AudioChannel,
    pub pending_sfx: Vec<SoundEffect>,
}

impl SFXSystem {
    pub fn play_sfx(&mut self, effect: SoundEffect) {
        self.pending_sfx.push(effect);
    }
}

#[derive(Clone, Debug)]
pub enum SoundEffect {
    Text,
    SanityUp,
    SanityDown,
    Pickup,
    EntranceExit,
    CashRegister,
}

pub fn setup_sfx(
    mut sfx_system: ResMut<SFXSystem>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    sfx_system.text = asset_server.load("sfx/01 text.mp3");
    sfx_system.sanity_up = asset_server.load("sfx/02 sanity up.mp3");
    sfx_system.sanity_down = asset_server.load("sfx/03 sanity down.mp3");
    sfx_system.pickup = asset_server.load("sfx/04 pickup.mp3");
    sfx_system.entrance_exit = asset_server.load("sfx/05 entrance_exit.mp3");
    sfx_system.cash_register = asset_server.load("sfx/06 cash register.mp3");
    sfx_system.channel = AudioChannel::new("sfx".to_owned());
    audio.set_volume_in_channel(0.1, &sfx_system.channel);
}

pub fn sfx_system(audio: Res<Audio>, mut sfx_system: ResMut<SFXSystem>) {
    let mut effects = sfx_system.pending_sfx.drain(..).collect::<Vec<_>>();

    for e in effects.drain(..) {
        let handle = match e {
            SoundEffect::Text => sfx_system.text.clone(),
            SoundEffect::SanityUp => sfx_system.sanity_up.clone(),
            SoundEffect::SanityDown => sfx_system.sanity_down.clone(),
            SoundEffect::Pickup => sfx_system.pickup.clone(),
            SoundEffect::EntranceExit => sfx_system.entrance_exit.clone(),
            SoundEffect::CashRegister => sfx_system.cash_register.clone(),
            _ => todo!(),
        };
        audio.play_in_channel(handle, &sfx_system.channel);
    }
}
