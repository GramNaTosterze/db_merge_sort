use rand::{prelude::Distribution, distributions::Standard, Rng};
use std::{fmt, str, num::ParseIntError};
use serde_derive::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn distance(self, point: Point) -> f64 {
        let x: f64 = self.x.abs_diff(point.x) as f64;
        let y: f64 = self.y.abs_diff(point.y) as f64;
        return (x.powf(2.0) + y.powf(2.0)).sqrt()
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl str::FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //(x,y)
        let point_str: Vec<&str> = s.trim_matches(|c| c == '('|| c == ')').split(",").collect();
        
        if point_str.len() != 2 {
            //error
        }

        Ok(Self{
            x:point_str[0].parse()?,
            y:point_str[1].parse()?
        })
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}


/* random */
impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        //return Point{x: rng.gen(), y: rng.gen()};
        return Point{x: rng.gen::<i32>()%32,y: rng.gen::<i32>()%32};
    }
}
