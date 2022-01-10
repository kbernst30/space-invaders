pub mod bus;
pub mod constants;
pub mod cpu;
pub mod emulator;
pub mod utils;

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::TextureCreator;

use crate::constants::*;
use crate::emulator::*;

fn main() {

    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Space Invaders", (DISPLAY_WIDTH * DISPLAY_FACTOR) as u32, (DISPLAY_HEIGHT * DISPLAY_FACTOR) as u32)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(DISPLAY_FACTOR as f32, DISPLAY_FACTOR as f32).unwrap();

    let mut creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, DISPLAY_WIDTH, DISPLAY_HEIGHT).unwrap();

    let mut emulator = Emulator::new();
    emulator.run();

    'running: loop {
        // texture.update(None, rusty_boy.get_screen(), 160 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    // rusty_boy.toggle_pause();
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    // rusty_boy.debug();
                },
                Event::KeyDown { keycode, .. } => {
                    // if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                    //     rusty_boy.set_button_state(*key);
                    // }
                }
                Event::KeyUp { keycode, .. } => {
                    // if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                    //     rusty_boy.reset_button_state(*key);
                    // }
                },
                _ => {}
            }
        }
    }
}
