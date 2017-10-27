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
use progconc::statistics::PerfMeasure;

// thread and sync primitives
use std::sync::{Mutex, Arc, Barrier};
use std::thread;
use std::time;
use std::sync::mpsc;


fn main() {
    // logger
    env_logger::init().unwrap();

    // parse arguments
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
        .get_matches();

    let pow_pers: usize = matches.value_of("pow_pers").map(|n| n.parse::<usize>().unwrap())
        .unwrap_or(6_usize);
    let scenario: usize = matches.value_of("scenario").map(|n| n.parse::<usize>()
        .unwrap()).unwrap_or(2_usize);
    let measure: bool = matches.is_present("measure");

    let nb_pers: usize = (2_usize).pow(pow_pers as u32);
    println!("Start simulation with \n {{ nb_pers = {} (2^{}), scenario = {}, measure = {} }}", nb_pers, pow_pers, scenario, measure);




    let measures: Option<(PerfMeasure, PerfMeasure)> = match scenario {
        0 => t1_algorithm_with_graph(nb_pers, measure),
        2 => t3_algorithm_with_graph(nb_pers, measure),
        _ => unimplemented!(),
    };

    if let Some((mb, ma)) = measures {
        println!("Memory before : {}MB", mb.get_maxrss_as_MB());
        println!("Memory after : {}MB", ma.get_maxrss_as_MB());
        println!("Memory usage : {}MB", ma.get_maxrss_as_MB() - mb.get_maxrss_as_MB());
        println!("Clock before : {}", mb.clock_t);
        println!("Clock after : {}", ma.clock_t);
        println!("Clock ticks : {}", ma.clock_t - mb.clock_t);
    }
}


fn t3_algorithm(nb_pers: usize, measure: bool) -> Option<(PerfMeasure, PerfMeasure)> {

    // ********* INITIALIZATION ********

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


    // measure 1 (before)
    let measure_before: Option<PerfMeasure> = if measure {
        Some(PerfMeasure::New())
    } else {
        None
    };

    // ********* ALGORITHM ********
    // start moving persons
    while (terrain.get_exited_cnt() < nb_pers) {
        // for each person
        for pers in persons.as_mut_slice() {
            if !pers.has_escaped {
                pers.look_and_move(&mut terrain);
            }
        }
        debug!("****** next turn ******", )
    }

    // measure 2
    let measure_after: Option<PerfMeasure> = if measure {
        Some(PerfMeasure::New())
    } else {
        None
    };

    if let (Some(mb), Some(ma)) = (measure_before,measure_after) {
        Some((mb, ma))
    } else {
        None
    }

}



fn t3_algorithm_with_graph(nb_pers: usize, measure: bool) -> Option<(PerfMeasure, PerfMeasure)> {

    // ********* INITIALIZATION ********
    // Initialize the terrain and place persons in it :
    // ********* INITIALIZATION ********
    let (mut terrain,
        mut persons) = initialize_terrain_and_users(nb_pers, XSIZE,YSIZE);

    // move Terrain to the mutex protected reference counted pointer
    let protected_terrain = Arc::new(Mutex::new(terrain));

    let (stop_graph_tx, stop_graph_rx) = mpsc::channel();

    // ********* GRAPH RELATED ********
    let pterrain = protected_terrain.clone();
    let graph_handle = spawn_graph_thread(pterrain, nb_pers);//, stop_graph_rx);
    // ********* GRAPH RELATED ********


    // measure 1 (before)
    let measure_before: Option<PerfMeasure> = if measure {
        Some(PerfMeasure::New())
    } else {
        None
    };

    // ********* ALGORITHM ********
    // start moving persons
    'running: while protected_terrain.lock().unwrap().get_exited_cnt() < nb_pers {
        // for each person
        for pers in persons.as_mut_slice() {
            if !pers.has_escaped {
                pers.look_and_move(&mut protected_terrain.lock().unwrap());
            }
        }
        debug!("****** next turn ******  {} have left the Terrain", protected_terrain.lock().unwrap().get_exited_cnt());
    }

    // measure 2
    let measure_after: Option<PerfMeasure> = if measure {
        Some(PerfMeasure::New())
    } else {
        None
    };

    stop_graph_tx.send(()); // kill graph thread

    graph_handle.join().unwrap();

    if let (Some(mb), Some(ma)) = (measure_before,measure_after) {
        Some((mb, ma))
    } else {
        None
    }
}





fn t1_algorithm_with_graph(nb_pers: usize, measure: bool) -> Option<(PerfMeasure, PerfMeasure)>  {

    // ********* INITIALIZATION ********
    let (mut terrain,
        mut persons) = initialize_terrain_and_users(nb_pers, XSIZE,YSIZE);

    // move Terrain to the mutex protected reference counted pointer
    let protected_terrain = Arc::new(Mutex::new(terrain));


    // measure 1 (before)
    let measure_before: Option<PerfMeasure> = if measure {
        Some(PerfMeasure::New())
    } else {
        None
    };

    // ********* THREAD DISTRIBUTION ********
    let mut person_thread_handles = Vec::with_capacity(nb_pers);
    let barrier = Arc::new(Barrier::new(nb_pers + 1));

    // create one thread per person
    while let Some(mut pers) = persons.pop() {

        let pterrain = protected_terrain.clone();
        //let tx = tx.clone();
        let c = barrier.clone();

        // Threads declaration :
        let handle = thread::spawn(move || {
            debug!("waiting {}", pers.id);
            c.wait();
            debug!("go ! {}", pers.id);
            while !pers.has_escaped {
                { // whole terrain is blocked
                    let mut my_terrain = pterrain.lock().unwrap();
                    pers.look_and_move(&mut my_terrain);
                } // locked mutex goes out of scope : terrain is availabe again
                //thread::sleep(time::Duration::from_millis(100));
            }
            debug!("I escaped : {}", pers.id);
        });
        person_thread_handles.push(handle);
    };

    // ********* GRAPH RELATED ********
    let pterrain = protected_terrain.clone();
    let graph_handle = spawn_graph_thread(pterrain, nb_pers);//, stop_graph_rx);
    // ********* GRAPH RELATED ********


    barrier.wait();


    for handle in person_thread_handles {
        handle.join().unwrap();
    };

    // measure 2
    let measure_after: Option<PerfMeasure> = if measure {
        Some(PerfMeasure::New())
    } else {
        None
    };


    graph_handle.join();


    if let (Some(mb), Some(ma)) = (measure_before,measure_after) {
        Some((mb, ma))
    } else {
        None
    }
}



