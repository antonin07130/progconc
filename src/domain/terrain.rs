use super::YSIZE;
use super::XSIZE;
use super::NBEXIT;
use super::Point;

use std::fmt; // formatting for console display


// *******
// TERRAIN
// *******
pub struct Terrain {
    data : [[isize; YSIZE]; XSIZE],
    exit_points : [Point; NBEXIT],
}

impl Terrain {

    // constructor
    pub fn new()-> Terrain {
        Terrain{ data: [[0; YSIZE]; XSIZE], exit_points : Terrain::create_exit_points() }
    }

    // constructor helper to create the exit
    fn create_exit_points() -> [Point; NBEXIT] {
        // top left corner
        [   Point{x:0, y: (YSIZE as isize) - 1 },
            Point{x:0, y: (YSIZE as isize) - 2 },
            Point{x:1, y: (YSIZE as isize) - 1 },
            Point{x:1, y: (YSIZE as isize) - 2 }
        ]
    }

    // add rectangular obstacles in the terrain. Poisitions are occupied by -1 values
    pub fn add_obstacle(&mut self, lower_left : Point, upper_right : Point ){
        for x in lower_left.x..upper_right.x + 1 {
            for y in lower_left.y..upper_right.y + 1 {
                self.data[x as usize][y as usize] = -1;
            }
        }
    }

    fn count_persons_in_terrain(&self) -> isize {
        let mut count: isize = 0;
        for i in 0..XSIZE {
            for j in 0..YSIZE {
                if (self.data[i][j] != 0) &&  self.data[i][j] != -1 { count = count + 1; };
            }
        }
        count
    }

    pub fn set_pt(&mut self, point: &Point, value : isize) {
        self.data[point.x as usize][point.y as usize] = value;
    }

    // take the value at src, and write it at dst, reset src to 0 ("free")
    pub fn move_src_to_dst(&mut self, src : &Point, dst : &Point) {
        self.data[dst.x as usize][dst.y as usize] = self.data[src.x as usize][src.y as usize];
        self.data[src.x as usize][src.y as usize] = 0; // "free" occupied point
    }


    // list possible moves around a certain Point
    pub fn list_possible_moves(&self, center: &Point) -> Vec<Point> {
        let mut result: Vec<Point> = Vec::new();

        // check all neighboors
        for x_prob in (center.x - 1)..(center.x + 2) {
            for y_prob in (center.y - 1)..(center.y + 2) {
                if self.check_valid(x_prob, y_prob) {
                    result.push(Point{x: x_prob, y: y_prob});
                    //println!("({},{}) is valid", x_prob, y_prob);
                } else {
                    //println!("({},{}) is not valid", x_prob, y_prob);
                }
            }
        }
        result
    }


    // check if some position is candidate to a move (not occupied nor an obstacle)
    fn check_valid(&self, x_prob: isize, y_prob: isize) -> bool {
        (x_prob >= 0 && x_prob < XSIZE as isize) && // check x_prob within Terrain bounds
            (y_prob >= 0 && y_prob < YSIZE as isize) && // check y_prob within Terrain bounds
            self.data[x_prob as usize][y_prob as usize] == 0 // check (x_pos, y_pos) is free
    }
}

impl fmt::Display for Terrain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Terrain {{\n");
        for y in (0..YSIZE).rev() {
            for x in 0..XSIZE {
                write!(f, "({},{})={} \t", x, y, self.data[x][y]);
            }
            write!(f, "\n");
        }
        write!(f, "}}\n")
    }
}
