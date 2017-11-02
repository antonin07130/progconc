extern crate rand;

use super::YSIZE;
use super::XSIZE;
use super::NBEXIT;
use super::Point;

use std::fmt; // formatting for console display
use self::rand::{Rng, SeedableRng};

// *******
// TERRAIN
// *******
pub struct Terrain {
    pub xsize : usize,
    pub ysize : usize,
    data : Vec<isize>,
    exit_points : [Point; NBEXIT],
    exited_cnt : usize,
}

impl Terrain {

    pub fn get_data_ref(&self) -> &Vec<isize> {
        &self.data
    }

    // constructor
    pub fn new(xsize: usize, ysize:usize)-> Terrain {
        let data : Vec<isize> = vec![0; xsize * ysize];//Vec::with_capacity(xsize * ysize);

        Terrain{ xsize, ysize, data,
        exit_points : Terrain::create_exit_points(),
        exited_cnt : 0 }
    }

    /// Creates a sample terrain with premade obstacles
    /// if YSIZE < 4 or XSIZE < 11 this function fails
    pub fn new_sample(xsize: usize, ysize:usize) -> Terrain {
        let mut terr = Terrain::new(xsize, ysize);
        let large_ll = Point{x: XSIZE as isize / 10 , y: 1};
        let large_ur = Point{x: XSIZE as isize / 10 * 2 , y: YSIZE as isize - 2};
        let long_ll  = Point{x: XSIZE as isize / 10 * 2 + 2 , y: YSIZE as isize / 5};
        let long_ur  = Point{x: XSIZE as isize / 10 * 9 -  1 , y: YSIZE as isize / 5 + 1};
        //println!("large_lb {},  lagre_rt {} ; long_lb {}, long_rt {}", large_lb, lagre_rt, long_lb, long_rt); // debug
        terr.add_obstacle(large_ll, large_ur); // large obstacle (takes lots of Y)
        terr.add_obstacle(long_ll, long_ur); // long obstacle (takes lot of X)
        terr
    }

    // constructor helper to create the exit
    fn create_exit_points() -> [Point; NBEXIT] {
        // top left corner
        [
            Point{x:0, y: (YSIZE as isize) - 1 },
            Point{x:0, y: (YSIZE as isize) - 2 },
            Point{x:1, y: (YSIZE as isize) - 1 },
            Point{x:1, y: (YSIZE as isize) - 2 }
        ]
    }

    pub fn get_exit_points(&self) -> &[Point; NBEXIT] {
        &self.exit_points
    }

    // add rectangular obstacles in the terrain. Poisitions are occupied by -1 values
    pub fn add_obstacle(&mut self, lower_left : Point, upper_right : Point ){
        for x in lower_left.x..upper_right.x + 1 {
            for y in lower_left.y..upper_right.y + 1 {
                self.set_pt_val(&Point{x,y}, -1);
            }
        }
    }


    /// Returns a random point on the Terrain that is available.
    pub fn get_random_free_point(&self) -> Option<Point> {
        //let mut rng = rand::thread_rng();
        let seed: &[_] = &[1,]; // declare random generator with constant seed to get consistant results between executions.
        let mut rng = rand::StdRng::from_seed(seed);


        let mut avl_points : Vec<Point> = Vec::new();

        for idx in 0..self.data.len() {
            if self.data[idx] == 0 {
                avl_points.push( self.get_point(idx));
            };
        }

        let sz = avl_points.len();
        if sz > 0 { // free positions remaining
            Some(avl_points.remove(rng.gen_range(0,sz)))
        } else { // no free position remaining
            None
        }
    }

    pub fn get_exited_cnt(&self) -> usize {
        self.exited_cnt
    }

    /// non thread safe count of persons in the terrain.
    pub fn count_persons_in_terrain(&self) -> usize {
        let mut count: usize = 0;
        for val in self.data.as_slice() {
            if (*val != 0) &&  *val != -1 { count = count + 1; }
        }
        count
    }

    fn get_offset(&self, point : &Point) ->usize {
        ( self.xsize * point.y as usize ) + point.x as usize
    }

    fn get_point(&self, offset : usize) -> Point{
        let x = (offset % self.xsize) as isize; // x coordinate is the remainder of offset / x
        let y = (offset / self.xsize) as isize; // y coordinate is the amount of times we have been through x
        Point{x, y}
    }

    pub fn set_pt_val(&mut self, point: &Point, value : isize) {
        let offset = self.get_offset(point);
        self.data[offset] = value;
    }

    pub fn get_pt_val(&self, point: &Point) -> isize {
        let offset = self.get_offset(point);
        self.data[offset]
    }

    /// take the value at src, and write it at dst, reset src to 0 ("free")
    /// we shall make this function thread safe : no 2 moves at the same time
    pub fn move_src_to_dst(&mut self, src : &Point, dst : &Point) -> Option<()> {

        if self.get_pt_val(dst) != 0 { // Trying to move to an occupied position
             return None // no move and early exit
        } else if self.exit_points.contains(dst) { // do not change the value of exit points
            self.exited_cnt = self.exited_cnt + 1;
            debug!("terrain exits :{}", self.exited_cnt );
        } else {
            let val = self.get_pt_val(src);
            self.set_pt_val(dst,val);
            //self.data[dst.x as usize][dst.y as usize] = self.data[src.x as usize][src.y as usize];
        }
        self.set_pt_val(src,0);
        //self.data[src.x as usize][src.y as usize] = 0; // "free" occupied point
        Some(()) // some  move
    }


    /// list possible moves in the neighborhood of a certain Point
    /// Caution : this function is only valid if no move happens in the neighborhood while reading
    pub fn list_possible_moves(&self, center: &Point) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::with_capacity(8);

        // check all neighboors
        for x_prob in (center.x - 1)..(center.x + 2) {
            for y_prob in (center.y - 1)..(center.y + 2) {
                if self.check_valid(x_prob, y_prob) {
                    result.push(Point{x: x_prob, y: y_prob});
                    debug!("({},{}) is valid", x_prob, y_prob);
                } else {
                    debug!("({},{}) is not valid", x_prob, y_prob);
                }
            }
        }
        result
    }


    // check if some position is candidate to a move (not occupied nor an obstacle)
    fn check_valid(&self, x_prob: isize, y_prob: isize) -> bool {
        (x_prob >= 0 && x_prob < XSIZE as isize) && // check x_prob within Terrain bounds
            (y_prob >= 0 && y_prob < YSIZE as isize) && // check y_prob within Terrain bounds
            self.get_pt_val(&Point{x: x_prob, y: y_prob}) ==0
               // .get.data[x_prob as usize][y_prob as usize] == 0 // check (x_pos, y_pos) is free
    }


    fn check_valid_pt(&self, prob_point : &Point) -> bool {
        (prob_point.x >= 0 && prob_point.x < self.xsize as isize) && // check x_prob within Terrain bounds
            (prob_point.y >= 0 && prob_point.y < self.ysize as isize) && // check y_prob within Terrain bounds
            self.get_pt_val(prob_point) == 0 // check (x_pos, y_pos) is free
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Terrain {{\n").unwrap();
        for y in (0..YSIZE).rev() {
            for x in 0..XSIZE {
                write!(f, "({},{})={} \t", x, y, self.get_pt_val(&Point{x : x as isize, y : y as isize})).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "}}\n")
    }
}
