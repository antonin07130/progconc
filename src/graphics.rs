
extern crate sdl2;
extern crate rand;

use self::sdl2::Sdl;
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

use std::sync::{Mutex, Arc};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::Receiver;


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


pub fn initialize_windows(x_size:usize, y_size:usize, sdl_context : & Sdl) -> (WindowCanvas, Vec<u8>,  TextureCreator<WindowContext>) {

    let num_px = sdl2::pixels::PixelFormatEnum::ARGB8888.byte_size_of_pixels(x_size * y_size);

    let mut pixels : Vec<u8> = vec![0; num_px];

    let video_subsystem = sdl_context.video().unwrap();

    let window: Window = video_subsystem
        .window("rust-sdl2 demo: Window", 800, 600)
        //.opengl()
        .resizable()
        .build()
        .unwrap();

    let mut canvas: WindowCanvas = window.into_canvas().accelerated().build().unwrap();
    debug!("Using SDL_Renderer \"{}\"", canvas.info().name);
    //let mut tick = 0;
    //let mut event_pump = sdl_context.event_pump().unwrap();

    let mut text_creator: TextureCreator<WindowContext> = canvas.texture_creator();

    return (canvas, pixels, text_creator)
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
            _ => false,
        };
    };

    false
}



pub fn update_texture(pixels :&mut Vec<u8>, terrain : &Terrain, canvas : &mut WindowCanvas, texture : &mut Texture) {
    let data = terrain.get_data_ref();
    let x_size = terrain.xsize;
    let y_size = terrain.ysize;
    let num_px = sdl2::pixels::PixelFormatEnum::ARGB8888.byte_size_of_pixels(x_size * y_size);
    assert_eq!(pixels.len() as usize, num_px);

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
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for idx in 0..(x_size * y_size) {
            let offset = idx * 4;
            unsafe{
            buffer[offset + 0] = *data.get_unchecked(idx) as u8; // b
            buffer[offset + 1] = (*data.get_unchecked(idx)*100) as u8;//(data.get_unchecked(idx)*100_isize) as u8; // g
            buffer[offset + 2] = *data.get_unchecked(idx) as u8; // r
            buffer[offset + 3] = 255_u8; // a (opaque)
            }
        }
    });
}

/// this function keeps on drawing *the same* terrain over and over again.
pub fn graph_loop(terrain: &Terrain){

    let data_array = terrain.get_data_ref();
    let x_size = terrain.xsize;
    let y_size = terrain.ysize;

    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().expect("My event PUMPPP!");

    let (mut canvas,
        mut pixels,
       // mut texture,
        mut text_creator) = initialize_windows(x_size, y_size, &sdl_context);



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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        //canvas.clear();
        canvas.present();
    }

}




pub fn spawn_graph_thread(pterrain : Arc<Mutex<Terrain>>, nb_pers : usize) -> JoinHandle<()> {


    let graph_handle = thread::spawn(move || {

        let xsize;
        let ysize;
        {
            let terrain = pterrain.lock().unwrap();
            xsize = terrain.xsize;
            ysize = terrain.ysize;
        }

        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump()
            .expect("Need an event pump !");
        //initialize graphs
        let (mut canvas,
            mut pixels,
            // mut texture,
            mut text_creator) = initialize_windows(
            xsize,
            ysize,
            &sdl_context);

        let mut texture = create_texture(&text_creator, xsize, ysize);

        // fps computation
        let mut start = Instant::now();
        let deltat_render = Duration::from_millis(33);

        let mut exited_count: usize = 0;

        'running: while exited_count < nb_pers { // measure of exited count is not correct

            //            // ********* GRAPH RELATED ********
            if start.elapsed().gt(&deltat_render) {
                debug!("from grph exited : {}", exited_count);

                // kill switch (check here to prevent blocking in noop loops)
//                if !stop_rx.try_iter().next().is_none() {
//                    println!("stopping me");
//                    break 'running;
//                }

                // do not render more than 30 fps
                start = Instant::now();
                {
                    // graph update
                    let terrain = pterrain.lock().unwrap();
                    update_texture(&mut pixels, &terrain, &mut canvas, &mut texture);
                    exited_count = terrain.get_exited_cnt();
                }
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                canvas.copy(&texture,
                            None,
                            None).unwrap();
            }
            canvas.present();


//            if check_quit(&mut event_pump){
//                break 'running;
//            }
        }
    });

    graph_handle
}



#[cfg(test)]
mod tests {
    use graphics::*;
    use super::sdl2;
    use super::sdl2::Sdl;
    use super::sdl2::EventPump;
    use super::sdl2::event::Event;
    use super::sdl2::keyboard::Keycode;
    use super::sdl2::VideoSubsystem;
    use super::sdl2::video::{Window, WindowContext};
    use super::sdl2::render::{Canvas, WindowCanvas};


    use std::thread;
    use std::thread::JoinHandle;

    #[test]
    fn test_event_loop_thread() {

        let graph_handle = thread::spawn(move || {
            let sdl_context = sdl2::init().unwrap();
            let mut event_pump = sdl_context.event_pump().expect("my event pump !");

            let video_subsystem = sdl_context.video().unwrap();

            let window: Window = video_subsystem
                .window("rust-sdl2 demo: Window", 800, 600)
                //.opengl()
                .resizable()
                .build()
                .unwrap();

            let mut canvas: WindowCanvas = window.into_canvas().accelerated().build().unwrap();

            'myloop: loop {
                canvas.present();

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'myloop,
                        _ => (),
                    };
                };
            }
        });

        graph_handle.join();
    }

    #[test]
    fn test_win() {

        test_disp();
    }
}
