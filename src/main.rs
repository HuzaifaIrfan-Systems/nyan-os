
use std::{thread};


mod console;
mod sound;



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
        console::animate();
    });

    // Thread for audio setup and playback
    let audio_handle = thread::spawn(|| {
        sound::set_volume();
        sound::play_audio();
    });

    // Wait for both threads to complete
    animation_handle.join().unwrap();
    audio_handle.join().unwrap();
}
