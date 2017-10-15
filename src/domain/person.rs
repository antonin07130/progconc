use super::Point;
use super::terrain::Terrain;

use std::fmt; // formatting for console display
use std::cmp::Ordering::Equal;


// ******
// PERSON
// ******
#[derive(Debug)]
pub struct Person {
    pub id: usize,
    pub position : Point,
}

impl Person {

    pub fn new_placed(terrain : &mut Terrain, id: usize, position : Point) -> Person {
        terrain.set_pt(&position, id as isize);
        Person {id, position}

    }

    pub fn new_unplaced(id: usize) -> Person {
        Person { id, position: Point { x: 0, y: 0 } }
    }

    pub fn new(id : usize, position : Point) -> Person {
        Person { id, position }
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

    pub fn place_on_terrain(&self, terrain: &mut Terrain){
        terrain.set_pt(&self.position, self.id as isize);
    }


    pub fn move_to(&mut self, terrain: &mut Terrain, new_point: &Point) {
        terrain.move_src_to_dst(&self.position, new_point);
        self.position.x = new_point.x; // change internal position (copy of x and y)
        self.position.y = new_point.y;
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person : {{ id : {}, position : {} }}", self.id, self.position)
    }
}
