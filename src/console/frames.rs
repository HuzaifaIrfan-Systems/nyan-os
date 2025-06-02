use crate::console::color::Color;

mod nyan_01;
mod nyan_02;
mod nyan_03;

mod nyan_04;
mod nyan_05;
mod nyan_06;

mod nyan_07;
mod nyan_08;
mod nyan_09;

mod nyan_10;
mod nyan_11;
mod nyan_12;


pub const FRAMES: [[[Color; 80]; 25]; 12] = [
    nyan_01::NYAN_01,
    nyan_02::NYAN_02,
    nyan_03::NYAN_03,
    nyan_04::NYAN_04,
    nyan_05::NYAN_05,
    nyan_06::NYAN_06,
    nyan_07::NYAN_07,
    nyan_08::NYAN_08,
    nyan_09::NYAN_09,
    nyan_10::NYAN_10,
    nyan_11::NYAN_11,
    nyan_12::NYAN_12,
];
