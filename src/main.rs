extern crate progconc;
extern crate clap;

use clap::{Arg, App};
use progconc::domain::Point;
use progconc::domain::terrain::Terrain;
use progconc::domain::person::Person;

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
        .get_matches();

    let pow_pers  : usize = matches.value_of("pow_pers").map(|n| n.parse::<usize>().unwrap()).unwrap_or(4_usize);
    let nb_pers   : usize = (2_usize).pow(pow_pers as u32);
    let scenario  : usize = matches.value_of("scenario").map(|n| n.parse::<usize>().unwrap()).unwrap_or(2_usize);
    let measure   : bool  = matches.value_of("measure").map(|n| n.parse::<bool>().unwrap()).unwrap_or(false);

    println!("Start simulation with \n {{ nb_pers = {} (2^{}), scenario = {}, measure = {} }}", nb_pers, pow_pers, scenario, measure );

    match scenario {
        2 => t3_algorithm(nb_pers,measure),
        _ => unimplemented!(),
    };


}



fn t3_algorithm(nb_pers : usize, measure : bool) {
    // Initialize the terrain and place persons in it :
    let mut terrain: Terrain = Terrain::new_sample();
    #[derive(Debug)]
    let mut persons : Vec<Person>= Vec::with_capacity(nb_pers as usize);

    for i in 1..nb_pers+1 {
        let pt : Point = terrain.get_random_free_point();
        let new_pers = Person::new( i, pt);
        terrain.set_pt(&new_pers.position, new_pers.id as isize); // occupy }
        println!("placing : {}", &new_pers);
        persons.push(new_pers);
    }
    println!(" pers in terrain : {}", terrain.count_persons_in_terrain() );
    println!(" expected pers in terrain : {}", nb_pers);
    assert!(terrain.count_persons_in_terrain() == nb_pers);
    println!("persons array : {:?}", persons );
    
    println!("Initial terrain \n {}", terrain );

    // start moving persons
    while (terrain.get_exited_cnt() < nb_pers) {
        // for each person
        for pers in persons.as_mut_slice() {
            // select move
            let moves : Vec<Point>;
            println!("Dealing with : {}",&pers);
            moves = terrain.list_possible_moves(&pers.position);
            println!("Possible moves : {:?}", moves);
            #[derive(Debug)]
            let good_point = pers.choose_best_move(&moves);
            // move
            pers.move_to(&mut terrain, &good_point);
            println!("Moving to : {}", good_point);
        }
        println!("****** next turn ******", )
    }
        
    println!("Final terrain \n {}",terrain );

}