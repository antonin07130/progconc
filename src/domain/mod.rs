pub mod terrain;
pub mod person;

pub const XSIZE:  usize = 10;
pub const YSIZE:  usize = 5;
pub const NBEXIT: usize = 4;

use std::fmt; // formatting for console display
use std::cmp;

// *****
// POINT
// *****
#[derive(Debug)] // for debugging printing purpose
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