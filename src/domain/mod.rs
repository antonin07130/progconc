pub mod terrain;
pub mod person;

pub const XSIZE:  usize = 512;
pub const YSIZE:  usize = 128;
pub const NBEXIT: usize = 4;

use std::fmt; // formatting for console display
use std::cmp;

// *****
// POINT
// *****
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x : isize,
    pub y : isize
}

impl Point {
    fn square_distance_to(&self, other: &Point) -> f32 {
        // this is inccurate : we consider the bottom right of each point
        // but should be enough for our program.
        // we only want ordering feature : we wont compute the square root :
        ((other.x - self.x).pow(2) + (other.y - self.y).pow(2)) as f32
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }

    fn ne(&self, other: &Point) -> bool {
        !self.eq(other)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}

use domain::terrain::Terrain;
use domain::person::Person;

pub fn initialize_terrain_and_users(nb_pers: usize, xsize: usize, ysize: usize) -> (Terrain, Vec<Person>) {
    // ********* INITIALIZATION ********

    // Initialize the terrain and place persons in it :
    let mut terrain: Terrain = Terrain::new_sample(xsize, ysize);
    #[derive(Debug)]
    let mut persons: Vec<Person> = Vec::with_capacity(nb_pers as usize);

    for i in 1..nb_pers + 1 {
        let pt: Point = terrain.get_random_free_point()
            .expect("Not enough free positions on the Terrain for all Persons");
        let mut new_pers = Person::new(i*10, pt);
        new_pers.place_on_terrain(&mut terrain);
        //terrain.set_pt(&new_pers.position, new_pers.id as isize); // occupy }
        debug!("placing : {}", &new_pers);
        persons.push(new_pers);
    }
    info!(" pers in terrain : {}", terrain.count_persons_in_terrain());
    debug!(" expected pers in terrain : {}", nb_pers);
    assert_eq!(terrain.count_persons_in_terrain(), nb_pers);
    debug!("persons array : {:?}", persons);

    (terrain, persons)
}