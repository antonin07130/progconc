extern crate sdl2;


use self::sdl2::rect::{Point, Rect};
use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::mouse::MouseButton;
use self::sdl2::keyboard::Keycode;
use self::sdl2::video::{Window, WindowContext};
use self::sdl2::render::{Canvas, Texture, TextureCreator};
// use game_of_life::{SQUARE_SIZE, PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT};


pub fn create_windows() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("rust-sdl2 demo: Game of Life",
                512,
                128)
        .position_centered()
        .build()
        .unwrap();

        use std::{thread, time};

    let ten_millis = time::Duration::from_secs(1); // to be sure to see it
    let now = time::Instant::now();

    thread::sleep(ten_millis);

}