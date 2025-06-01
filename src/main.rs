use rodio::{Decoder, OutputStream, Sink, Source}; // <- Added Source trait here
use std::io::Cursor;
use std::io::{Write, stdout};

use alsa::mixer::{Mixer, SelemChannelId, SelemId};

pub mod color;
pub mod frames;
use crate::color::Color;

use std::{thread, time::Duration};

use crossterm;

fn set_volume() {
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

fn play_audio() {
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

fn draw_frame(frame: Vec<Vec<Color>>) {
    let padding_top = 10;
    for (i, row) in frame.iter().enumerate() {
        // Move cursor to beginning of line with vertical position
        // Using i (the row index) for vertical positioning
        print!("\x1B[{};{}H", i + padding_top, 1);

        for color in row.iter() {
            print!("{}", color.to_ansi_code());
        }
    }
    stdout().flush().unwrap();
}

pub fn resize_frames(
    original: [[[Color; 80]; 25]; 12],
    term_width: usize,
    term_height: usize,
) -> Vec<Vec<Vec<Color>>> {
    let _ = term_height;
    let old_width = 80;
    let old_height = 25;
    let new_width = term_width / 2;

    // Maintain aspect ratio
    let aspect_ratio = old_height as f32 / old_width as f32;
    let new_height = (new_width as f32 * aspect_ratio).round() as usize;

    original
        .iter()
        .map(|frame| {
            let mut new_frame = vec![vec![Color::Blue; new_width]; new_height];

            for y_new in 0..new_height {
                for x_new in 0..new_width {
                    let x_old = x_new * old_width / new_width;
                    let y_old = y_new * old_height / new_height;
                    new_frame[y_new][x_new] = frame[y_old][x_old];
                }
            }

            new_frame
        })
        .collect()
}

fn animate() {
    let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 25));
    println!("Term Size to {}x{}", term_width, term_height);
    let resized_frames = resize_frames(frames::FRAMES, term_width as usize, term_height as usize);

    thread::sleep(Duration::from_millis(2000));

    loop {
        for frame in resized_frames.iter() {
            // Move cursor to second row, first column
            draw_frame(frame.to_vec());
            // Delay between frames (e.g., 100 milliseconds)
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn main() {
    // Clear the screen (optional, for better animation effect)
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes to clear screen and move cursor to top-left
    println!("Nyan OS");
    println!("01 June 2025");
    println!("Developed by Huzaifa Irfan");
    println!("CTRL + ALT + F1 for NYAN");
    println!("CTRL + ALT + F2 for alsamixer");


    // Thread for animation
    let animation_handle = thread::spawn(|| {
        animate();
    });

    // Thread for audio setup and playback
    let audio_handle = thread::spawn(|| {
        set_volume();
        play_audio();
    });

    // Wait for both threads to complete
    animation_handle.join().unwrap();
    audio_handle.join().unwrap();
}
