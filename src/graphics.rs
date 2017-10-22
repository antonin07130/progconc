
extern crate sdl2;
extern crate rand;

use self::sdl2::rect::{Point, Rect};
use self::sdl2::pixels::Color;
use self::sdl2::EventPump;
use self::sdl2::event::Event;
use self::sdl2::mouse::MouseButton;
use self::sdl2::keyboard::Keycode;
use self::sdl2::VideoSubsystem;
use self::sdl2::video::{Window, WindowContext};
use self::sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};
// use game_of_life::{SQUARE_SIZE, PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT};

use ::domain::Point as myPoint;
use ::domain::terrain::Terrain;
use ::domain::person::Person;


use std::time::{Duration, Instant};


use self::rand::{Rng, SeedableRng, StdRng};


pub fn test_disp() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window : Window = video_subsystem
        .window("rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();


    let mut tick = 0;

    let now = Instant::now();
    while now.elapsed().as_secs() < 1_u64 { // displays the window for 1 second

    let mut event_pump = sdl_context.event_pump().unwrap();
        {
            // Update the window title.
            let mut window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!("Window - pos({}x{}), size({}x{}): {}",
                                position.0,
                                position.1,
                                size.0,
                                size.1,
                                tick);
            window.set_title(&title).unwrap();

            tick += 1;

        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas
        canvas.clear();
        canvas.present();
    }
}




pub fn initialize_windows(terrain : &Terrain) -> (WindowCanvas, Vec<u8>,  TextureCreator<WindowContext> , EventPump) {
    let data_array = terrain.get_data_ref();
    let x_size = terrain.xsize;
    let y_size = terrain.ysize;
    let mut pixels : Vec<u8> = vec![0;x_size * y_size * 4];

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem
        .window("rust-sdl2 demo: Window", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().present_vsync().build().unwrap();
    let mut tick = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut text_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    //texture.alpha_mod();
    //let texture = graphics::sdl2::render::Texture;
    return (canvas, pixels, text_creator, event_pump)
}

pub fn create_texture(text_creator : & TextureCreator<WindowContext>, width : usize, height : usize) -> Texture {
        text_creator.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::ARGB8888,
            width as u32,
            height as u32).unwrap()
    }

pub fn check_quit(event_pump : &mut EventPump) -> bool{
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return true,
            _ => {}
        }
    }
    false
}

//
//
//pub fn get_texture_upd_fn( data : &[&[isize]], x_size : usize, ysize : usize) -> FnOnce {
//    let mut rng = rand::thread_rng();
//
//    let fct = |buffer: &mut [u8], pitch: usize| {
//        for x in 0..x_size {
//            for y in 0..y_size {
//                let offset = ( x_size * 4 * y ) + x * 4;
//                buffer[offset + 0] = (data[x][y]*50_isize) as u8; // b
//                buffer[offset + 1] = (data[x][y]*100_isize) as u8; // g
//                buffer[offset + 2] = data[x][y] as u8; // r
//                buffer[offset + 3] = 255_u8; // a (opaque)
//            }
//        }
//        // graph running indicator (changes color at each frame)
//        buffer[0] = rng.next_u32() as u8; // b
//        buffer[1] = rng.next_u32()  as u8; // g
//        buffer[2] = rng.next_u32()  as u8; // r
//        buffer[3] = 255_u8; // a
//    };
//    return fct;
//}


pub fn update_texture(pixels :&mut Vec<u8>, terrain : &Terrain, canvas : &mut WindowCanvas, texture : &mut Texture) {
    let data = terrain.get_data_ref();
    let x_size = terrain.xsize;
    let y_size = terrain.ysize;
//    let mut rng = rand::thread_rng();
    {
        // Update the window title.
        let mut window = canvas.window_mut();

        let position = window.position();
        let size = window.size();
        let title = format!("Window - pos({}x{}), size({}x{})",
                            position.0,
                            position.1,
                            size.0,
                            size.1,
                            );
        window.set_title(&title).unwrap();
    }

    // -----
    // update texture
    //canvas.set_draw_color(Color::RGBA(0,0,0,0));
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for idx in 0..(x_size * y_size) {
            let offset = idx * 4;
            unsafe{
            buffer[offset + 0] = *data.get_unchecked(idx) as u8; // b
            buffer[offset + 1] = (data.get_unchecked(idx)*100_isize) as u8; // g
            buffer[offset + 2] = *data.get_unchecked(idx) as u8; // r
            buffer[offset + 3] = 255_u8; // a (opaque)
            }
        }

//        for x in 0..x_size {
//            for y in 0..y_size {
//                let offset = ( x_size * 4 * y ) + x * 4;
//                buffer[offset + 0] = (data[x][y]*50_isize) as u8; // b
//                buffer[offset + 1] = (data[x][y]*100_isize) as u8; // g
//                buffer[offset + 2] = data[x][y] as u8; // r
//                buffer[offset + 3] = 255_u8; // a (opaque)
//            }
//        }

//        // graph running indicator (changes color at each frame)
//        buffer[0] = rng.next_u32() as u8; // b
//        buffer[1] = rng.next_u32()  as u8; // g
//        buffer[2] = rng.next_u32()  as u8; // r
//        buffer[3] = 255_u8; // a
    });
}

/// this function keeps on drawing *the same* terrain over and over again.
pub fn graph_loop(terrain: &Terrain){

    let (mut canvas,
        mut pixels,
       // mut texture,
        mut text_creator,
        mut event_pump) = initialize_windows(terrain);

    let data_array = terrain.get_data_ref();
    let x_size = terrain.xsize;
    let y_size = terrain.ysize;

    let mut texture = create_texture(&text_creator, x_size, y_size);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        {
            update_texture(&mut pixels, terrain, &mut canvas, &mut texture);
        }


        canvas.copy(&texture,
                    None,
                    None).unwrap();

        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();
        canvas.present();
    }

}
