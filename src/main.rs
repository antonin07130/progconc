extern crate progconc;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;

// arguments parsing
use clap::{Arg, App};
// domain objects
use progconc::domain::*;
use progconc::domain::Point;
use progconc::domain::terrain::Terrain;
use progconc::domain::person::Person;
// graphic lib wrappers
use progconc::graphics::*;
// statistics lib wrapper
use progconc::statistics::Rusage;



fn main() {
    // Define and read command line arguments.
    let matches = App::new("progconc")
        .version("0.0.1")
        .author("Antonin Perrot-Audet <antonin.perrotaudet@yahoo.com>")
        .about("test concurrent capabilities of Rust language")
        .arg(Arg::with_name("pow_pers")
            .short("p")
            .long("persons")
            .takes_value(true)
            .help("The number of persons to generate, the program will create 2^p Persons"))
        .arg(Arg::with_name("scenario")
            .short("t")
            .long("scenario")
            .takes_value(true)
            .help("The scenario to use : 0 -> 1 thread per Person, 1 -> 4 threads to manage the Terrain, 2 -> mono-threaded"))
        .arg(Arg::with_name("measure")
            .short("m")
            .long("measure")
            .help("turns on performance measurement for the selected scenario"))
        //        .arg(Arg::with_name("debug")
        //            .short("d")
        //            .long("debug")
        //            .help("turns on verbose debug on standard output (very slow)"))
        .get_matches();

    let pow_pers: usize = matches.value_of("pow_pers").map(|n| n.parse::<usize>().unwrap())
        .unwrap_or(6_usize);
    let scenario: usize = matches.value_of("scenario").map(|n| n.parse::<usize>()
        .unwrap()).unwrap_or(2_usize);
    let measure: bool = matches.value_of("measure").map(|n| n.parse::<bool>()
        .unwrap()).unwrap_or(false);
    //    let debug     : bool  = matches.value_of("debug").map(|n| n.parse::<bool>()
    //        .unwrap()).unwrap_or(false);

    let nb_pers: usize = (2_usize).pow(pow_pers as u32);
    println!("Start simulation with \n {{ nb_pers = {} (2^{}), scenario = {}, measure = {} }}", nb_pers, pow_pers, scenario, measure);


    env_logger::init().unwrap();

    let res_before : Rusage  = Rusage::New();

    match scenario {
        2 => t3_algorithm_with_graph(nb_pers, measure),
        _ => unimplemented!(),
    };

    let res_after : Rusage = Rusage::New();

    println!("Memory before : {}MB",  res_before.get_maxrss_as_MB());
    println!("Memory after : {}MB", res_after.get_maxrss_as_MB());
    println!("Memory usage : {}MB", res_after.get_maxrss_as_MB() - res_before.get_maxrss_as_MB());

}


fn t3_algorithm(nb_pers: usize, measure: bool) {
    // Initialize the terrain and place persons in it :
    let mut terrain: Terrain = Terrain::new_sample(XSIZE, YSIZE);
    #[derive(Debug)]
    let mut persons: Vec<Person> = Vec::with_capacity(nb_pers as usize);

    for i in 1..nb_pers + 1 {
        let pt: Point = terrain.get_random_free_point()
            .expect("Not enough fee positions on the Terrain for all Persons");

        let mut new_pers = Person::new(i * 100, pt);
        new_pers.place_on_terrain(&mut terrain);
        //terrain.set_pt(&new_pers.position, new_pers.id as isize); // occupy }
        debug!("placing : {}", &new_pers);
        persons.push(new_pers);
    }
    println!(" Persons in terrain : {}", terrain.count_persons_in_terrain());
    debug!(" expected pers in terrain : {}", nb_pers);
    assert_eq!(terrain.count_persons_in_terrain(), nb_pers);
    debug!("persons array : {:?}", persons);

    // start moving persons
    while (terrain.get_exited_cnt() < nb_pers) {
        // for each person
        for pers in persons.as_mut_slice() {
            // select move
            let moves: Vec<Point>;
            debug!("Dealing with : {}", &pers);
            moves = terrain.list_possible_moves(&pers.position);
            debug!("Possible moves : {:?}", moves);
            #[derive(Debug)]
            let good_point = pers.choose_best_move(&moves);
            // move
            pers.move_to(&mut terrain, &good_point);
            debug!("Moving to : {}", good_point);
            if pers.has_escaped {

                // let's remove this person form the next computation loop
                //persons.remove_item(pers);
            }
        }
        debug!("****** next turn ******", )
    }


}


extern crate sdl2;

use self::sdl2::rect::Rect;
use self::sdl2::pixels::Color;
use self::sdl2::EventPump;
use self::sdl2::event::Event;
use self::sdl2::mouse::MouseButton;
use self::sdl2::keyboard::Keycode;
use self::sdl2::VideoSubsystem;
use self::sdl2::video::{Window, WindowContext};
use self::sdl2::render::{Canvas, Texture, TextureCreator, WindowCanvas};


fn t3_algorithm_with_graph(nb_pers: usize, measure: bool) {
    // Initialize the terrain and place persons in it :
    let mut terrain: Terrain = Terrain::new_sample(XSIZE, YSIZE);
    #[derive(Debug)]
    let mut persons: Vec<Person> = Vec::with_capacity(nb_pers as usize);

    for i in 1..nb_pers + 1 {
        let pt: Point = terrain.get_random_free_point()
            .expect("Not enough fee positions on the Terrain for all Persons");

        let mut new_pers = Person::new(i * 100, pt);
        new_pers.place_on_terrain(&mut terrain);
        //terrain.set_pt(&new_pers.position, new_pers.id as isize); // occupy }
        debug!("placing : {}", &new_pers);
        persons.push(new_pers);
    }
    println!(" pers in terrain : {}", terrain.count_persons_in_terrain());
    debug!(" expected pers in terrain : {}", nb_pers);
    assert_eq!(terrain.count_persons_in_terrain(), nb_pers);
    debug!("persons array : {:?}", persons);

    println!("Initial terrain \n {}", terrain);


    // ********* GRAPH RELATED ********
    //initialize graphs
    let (mut canvas,
        mut pixels,
        // mut texture,
        mut text_creator,
        mut event_pump) = initialize_windows(&terrain);

    let mut texture = create_texture(&text_creator, progconc::domain::XSIZE, progconc::domain::YSIZE);
    // ********* GRAPH RELATED ********

    // start moving persons
    'running: while terrain.get_exited_cnt() < nb_pers {
        // for each person
        for pers in persons.as_mut_slice() {
            if !pers.has_escaped {
                // ********* GRAPH RELATED ********
                //canvas.clear();
                // ********* GRAPH RELATED ********

                // select move
                let moves: Vec<Point>;
                debug!("Dealing with : {}", &pers);
                moves = terrain.list_possible_moves(&pers.position);
                debug!("Possible moves : {:?}", moves);
                #[derive(Debug)]
                let good_point = pers.choose_best_move(&moves);
                // move
                pers.move_to(&mut terrain, &good_point);

                debug!("Moving to : {}", good_point);


                // ********* GRAPH RELATED ********
                // graph update
                {
                    //texture.with_lock(None, get_texture_upd_fn())

                    update_texture(&mut pixels, &terrain, &mut canvas, &mut texture);
                }

                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                canvas.copy(&texture,
                            None,
                            None).unwrap();
                //canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.present();

                if check_quit(& mut event_pump) {
                    break 'running;
                }
                // ********* GRAPH RELATED ********
            }
        }

        debug!("****** next turn ****** excited : {}", terrain.get_exited_cnt());
    }

    println!("Final terrain \n {}", terrain);

    //-----
    //graph_loop(&terrain);
    //------
}




