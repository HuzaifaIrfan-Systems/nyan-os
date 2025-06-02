use rodio::{Decoder, OutputStream, Sink, Source}; // <- Added Source trait here
use alsa::mixer::{Mixer, SelemChannelId, SelemId};

use std::io::Cursor;

pub fn set_volume() {
    // Open the "default" mixer
    let mixer = Mixer::new("default", false).expect("Failed to open mixer");

    // Target the "Master" playback volume
    let sid = SelemId::new("Master", 0);
    let selem = mixer
        .find_selem(&sid)
        .expect("Could not find 'Master' control");

    // Set volume on both left and right channels (or Front Left / Front Right)
    let min = selem.get_playback_volume_range().0;
    let max = selem.get_playback_volume_range().1;

    // Let's say we want to set the volume to 50% of max
    let vol = min + (max - min) / 2;

    let _ = selem.set_playback_switch(SelemChannelId::FrontLeft, 1);
    let _ = selem.set_playback_switch(SelemChannelId::FrontRight, 1);

    selem
        .set_playback_volume(SelemChannelId::FrontLeft, vol)
        .expect("Failed to set volume");
    selem
        .set_playback_volume(SelemChannelId::FrontRight, vol)
        .expect("Failed to set volume");

    println!("Volume set to 50%");
}

pub fn play_audio() {
    // Get the default output stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create a sink
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Embed .wav file in binary
    let wav_data = include_bytes!("audio/nyan.wav");

    // Decode from memory
    let cursor = Cursor::new(wav_data.as_ref());
    let source = Decoder::new(cursor).unwrap();

    // Loop audio infinitely
    sink.append(source.repeat_infinite());

    // Keep playing
    sink.sleep_until_end();
}
