extern crate sdl2;

use sdl2::pixels::Color;
use std::{thread, time};
use std::intrinsics::size_of;
use std::ptr::null;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const SCREEN_PITCH: u32 = SCREEN_WIDTH * SCREEN_HEIGHT * 4;
const SCREEN_ASPECT_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT as f32;


struct DisplayManager {}


impl DisplayManager {
    pub fn new() -> Self {
        DisplayManager {}
    }
    pub fn startSDL() {
        let sdl2_context = sdl2::init().unwrap();
        let video = sdl2_context.video().unwrap();
        let window = video
            .window("Arcade Shooter", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        if sdl2_context == null() || video == null() || window == null {
            return;
        }
    }
}