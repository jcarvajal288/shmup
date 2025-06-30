use bevy::asset::{AssetServer, Handle};
use bevy::audio::{AudioPlayer, AudioSource, GlobalVolume, PlaybackSettings, Volume};
use bevy::prelude::{Commands, Event, EventReader, Res, ResMut, Resource};

pub enum SoundEffect {
    MenuSelect
}

#[derive(Event)]
pub struct PlaySoundEvent(pub SoundEffect);

#[derive(Resource, Default)]
pub struct Sounds {
    pub menu_select: Handle<AudioSource>
}

pub fn load_sounds(
    mut volume: ResMut<GlobalVolume>,
    mut sounds: ResMut<Sounds>, asset_server: Res<AssetServer>
) {
    volume.volume = Volume::new(0.2);
    sounds.menu_select = asset_server.load("sfx/se_select00.wav");
}

pub fn listen_for_play_sound_events(
    mut commands: Commands,
    mut play_sound_event_reader: EventReader<PlaySoundEvent>,
    sounds: Res<Sounds>,
) {
    for play_sound_event in play_sound_event_reader.read() {
        match play_sound_event.0 {
            SoundEffect::MenuSelect => {
                commands.spawn((
                    AudioPlayer::new(sounds.menu_select.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}