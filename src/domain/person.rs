use super::Point;
use super::terrain::Terrain;

use std::fmt; // formatting for console display
use std::cmp;
use std::cmp::Ordering::Equal;


// ******
// PERSON
// ******
#[derive(Debug)]
pub struct Person {
    pub id: usize,
    pub position : Point,
    pub has_escaped : bool,
}

impl Person {

    pub fn new_placed(terrain : &mut Terrain, id: usize, position : Point) -> Person {
        terrain.set_pt_val(&position, id as isize);
        Person {id, position, has_escaped :false}

    }

    pub fn new_unplaced(id: usize) -> Person {
        Person { id, position: Point { x: 0, y: 0 }, has_escaped : true }
    }

    pub fn new(id : usize, position : Point) -> Person {
        Person { id, position, has_escaped : false }
    }


    // Select the best available move that reduces most the distance to the azimuth point
    // or stay where you are.
    pub fn choose_best_move (&self, possible_moves: &Vec<Point>) -> Point {
        let azimuth: Point = Point{x: -2, y: 130};

        #[derive(Debug)] // to allow println for debugging purposes.
        let mut moves_and_dist: Vec<(&Point, f32)> = possible_moves.iter()
            .map(|x| (x, x.square_distance_to( &azimuth)))
            .collect();

        moves_and_dist.sort_by(
            |x, y| { x.1.partial_cmp((&y.1))
                .unwrap_or(Equal)
            }
        );
        //println!("debug sort :{:?}",moves_and_dist); // debug
        match moves_and_dist.first() {
            Some(&(ref point, _)) => Point{x:point.x, y:point.y},
            None => Point{x: self.position.x, y: self.position.y}, // todo : stay where you are for now...
        }
    }

    pub fn place_on_terrain(&mut self, terrain: &mut Terrain){
        terrain.set_pt_val(&self.position, self.id as isize);
        self.has_escaped = false;
    }

    pub fn remove_from_terrain(&mut self, terrain: &mut Terrain){
        terrain.set_pt_val(&self.position, 0 as isize);
        self.has_escaped = true;
    }


    pub fn move_to(&mut self, terrain: &mut Terrain, new_point: &Point) {
        if self.has_escaped == true {
        } else if terrain.get_exit_points().contains(new_point) {
            terrain.move_src_to_dst(&self.position, new_point); // should just increase exit counts
            //println!("I escaped : {}", self.id);
            self.has_escaped = true;
            self.remove_from_terrain(terrain);
        } else {
        terrain.move_src_to_dst(&self.position, new_point);
        self.position.x = new_point.x; // change internal position (copy of x and y)
        self.position.y = new_point.y;
        }
    }


    /// This function encapulates a complete move for a person :
    /// from looking around to actually moving to another place
    /// (and mutating the Person and the Terrain).
    pub fn look_and_move(&mut self, terrain : &mut Terrain) {
        //println!("Dealing with : {}", self);

        // look around
        let moves = terrain.list_possible_moves(&self.position);

        // select the best point (hope that no-one took it while thinking)
        //println!("Possible moves : {:?}", moves);
        #[derive(Debug)]
        let good_point = self.choose_best_move(&moves);

        // move to the best point
        if good_point != self.position {
            //println!("Moving to : {}", good_point);
            self.move_to(terrain, &good_point);
        } else {
            //println!("I, {}  am staying here : {}", self.id, good_point);
        }
    }
}

impl cmp::PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        (self.id == other.id)
    }

    fn ne(&self, other: &Person) -> bool {
        !self.eq(other)
    }
}


impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person : {{ id : {}, position : {} }}", self.id, self.position)
    }
}
