
use std::{thread, time::Duration};


use std::io::{Write, stdout};

use crossterm;


pub mod color;
pub mod frames;

use color::Color;
use frames::FRAMES;



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

pub fn animate() {
    let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 25));
    println!("Term Size to {}x{}", term_width, term_height);
    let resized_frames = resize_frames(FRAMES, term_width as usize, term_height as usize);

    loop {
        for frame in resized_frames.iter() {
            // Move cursor to second row, first column
            draw_frame(frame.to_vec());
            // Delay between frames (e.g., 100 milliseconds)
            thread::sleep(Duration::from_millis(100));
        }
    }
}