pub mod nyan_01;
pub mod nyan_02;
pub mod nyan_03;

pub mod nyan_04;
pub mod nyan_05;
pub mod nyan_06;

pub mod nyan_07;
pub mod nyan_08;
pub mod nyan_09;

pub mod nyan_10;
pub mod nyan_11;
pub mod nyan_12;

use crate::color::Color;

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
