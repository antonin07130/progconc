extern crate progconc;
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;

// domain objects
use progconc::domain::*;

// graphic lib wrappers
#[cfg(feature = "gui")]
use progconc::graphics::*;

// statistics lib wrapper
use progconc::statistics::{PerfMeasure, PerfResult};

// arguments parsing
use clap::{Arg, App};

// thread and sync primitives
use std::sync::{Mutex, Arc, Barrier};
use std::thread;
use std::io::Write;
use std::io;

fn main() {
    // logger
    env_logger::init().unwrap();

    #[cfg(feature = "gui")]
    println!("Compiled with gui feature on ☺ : turning off measure turns on rendering");
    #[cfg(not(feature = "gui"))]
    println!("Compiled without gui feature ☹ : measure turned on by default");


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

    if measure {
        let mut measures: Vec<PerfResult> = Vec::with_capacity(5);
        for i in 0..5 {
            print!("\rSimulation {}/5", i + 1);
            io::stdout().flush().unwrap();

            let measure = do_one_simulation(nb_pers, pow_pers, scenario, measure)
                .expect("No measure returned by this simulation : something went wrong");
            info!("Measure result : \n {}", measure);
            measures.push(measure);
        }
        println!();
        // compute mean measure and display it
        let medians = PerfResult::take_3_median_results(measures.as_slice());
        debug!("Removed outliers, kept :\n{:?}", medians);
        let mean = PerfResult::compute_mean_result(&medians);
        println!("Mean result for this simulation \n {} \n", mean);
    } else {
        do_one_simulation(nb_pers, pow_pers, scenario, measure);
    }
}


fn do_one_simulation(nb_pers: usize, pow_pers: usize, scenario: usize, measure: bool) -> Option<PerfResult> {

    // Select simulation to start according to option and compilaiton options
    let measures: Option<(PerfMeasure, PerfMeasure)> = match (scenario, measure) {
        (0, false) => {
            // algo 0, no measure : use gui if compiled
            #[cfg(feature = "gui")]
            let res = t0_algorithm_with_graph(nb_pers);
            #[cfg(not(feature = "gui"))]
            let res = t0_algorithm_perf(nb_pers);
            res
        }
        (0, true) => t0_algorithm_perf(nb_pers),
        (2, false) => {
            // algo 2, no measure : use gui if compiled
            #[cfg(feature = "gui")]
            let res = t3_algorithm_with_graph(nb_pers);
            #[cfg(not(feature = "gui"))]
            let res = t3_algorithm_perf(nb_pers);
            res
        }
        (2, true) => t3_algorithm_perf(nb_pers),
        _ => unimplemented!(),
    };

    // returns measurements from the simulation as a PerfResult
    if let Some((mb, ma)) = measures {
        Some(ma.minus(&mb))
    } else {
        None
    }
}


fn t3_algorithm_perf(nb_pers: usize) -> Option<(PerfMeasure, PerfMeasure)> {
    info!("Initialization");
    // ********* INITIALIZATION ********
    // Initialize the terrain and place persons in it :
    // ********* INITIALIZATION ********
    let (terrain, mut persons) =
        initialize_terrain_and_users(nb_pers, XSIZE, YSIZE);

    // move Terrain to the mutex protected reference counted pointer
    let protected_terrain = Arc::new(Mutex::new(terrain));


    // measure 1 (before)
    info!("Initialization done, measure starts");
    let measure_before: PerfMeasure = PerfMeasure::new();

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
    let measure_after: PerfMeasure = PerfMeasure::new();
    info!("End of algorithm, measure stops");

    Some((measure_before, measure_after))
}


#[cfg(feature = "gui")]
fn t3_algorithm_with_graph(nb_pers: usize) -> Option<(PerfMeasure, PerfMeasure)> {
    // ********* INITIALIZATION ********
    // Initialize the terrain and place persons in it :
    // ********* INITIALIZATION ********
    let (terrain,
        mut persons) = initialize_terrain_and_users(nb_pers, XSIZE, YSIZE);

    // move Terrain to the mutex protected reference counted pointer
    let protected_terrain = Arc::new(Mutex::new(terrain));


    // ********* GRAPH RELATED ********
    let pterrain = protected_terrain.clone();
    let graph_handle = spawn_graph_thread(pterrain, nb_pers);//, stop_graph_rx);
    // ********* GRAPH RELATED ********

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

    graph_handle.join().unwrap();

    None // no measure to return
}


#[cfg(feature = "gui")]
fn t0_algorithm_with_graph(nb_pers: usize) -> Option<(PerfMeasure, PerfMeasure)> {
    // ********* INITIALIZATION ********
    let (terrain,
        mut persons) = initialize_terrain_and_users(nb_pers, XSIZE, YSIZE);

    // move Terrain to the mutex protected reference counted pointer
    let protected_terrain = Arc::new(Mutex::new(terrain));


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
                {
                    // whole terrain is blocked
                    let mut my_terrain = pterrain.lock().unwrap();
                    pers.look_and_move(&mut my_terrain);
                } // locked mutex goes out of scope : terrain is availabe again
                //thread::sleep(time::Duration::from_millis(100));
            }
            debug!("I escaped : {}", pers.id);
        });
        person_thread_handles.push(handle);
    };


    barrier.wait();

    // ********* GRAPH RELATED ********
    let pterrain = protected_terrain.clone();
    graph_loop(pterrain, nb_pers);
    // ********* GRAPH RELATED ********


    for handle in person_thread_handles {
        handle.join().unwrap();
    };


    None // no measure to return
}


fn t0_algorithm_perf(nb_pers: usize) -> Option<(PerfMeasure, PerfMeasure)> {
    info!("Initialization");
    // ********* INITIALIZATION ********
    let (terrain,
        mut persons) = initialize_terrain_and_users(nb_pers, XSIZE, YSIZE);

    // move Terrain to the mutex protected reference counted pointer
    let protected_terrain = Arc::new(Mutex::new(terrain));


    info!("Initialization done, measure starts");
    // measure 1 (before)
    let measure_before: PerfMeasure = PerfMeasure::new();

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
                {
                    // whole terrain is blocked
                    let mut my_terrain = pterrain.lock().unwrap();
                    pers.look_and_move(&mut my_terrain);
                } // locked mutex goes out of scope : terrain is availabe again
                //thread::sleep(time::Duration::from_millis(100));
            }
            debug!("I escaped : {}", pers.id);
        });
        person_thread_handles.push(handle);
    };

    barrier.wait(); // wait for everybody to be placed to start moving


    for handle in person_thread_handles {
        handle.join().unwrap();
    };

    // measure 2
    let measure_after: PerfMeasure = PerfMeasure::new();
    info!("End of algorithm, measure stops");


    Some((measure_before, measure_after))
}


