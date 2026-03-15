use crate::prelude::*;

pub fn sound_effect(sound: Handle<AudioSource>) -> impl Bundle {
    (
        Name::new("Sound effect"),
        AudioPlayer::new(sound),
        PlaybackSettings::DESPAWN,
    )
}
