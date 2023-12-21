use point::Point;
use rand::{prelude::Distribution, distributions::Standard};
use serde_derive::{Serialize, Deserialize};
use std::num::ParseIntError;
use std::{fmt, str};
use std::cmp::Ordering;

mod point;

#[derive(Copy, Clone, Serialize, Deserialize, Eq)]
pub struct Triangle {
    pub a: Point,
    b: Point,
    c: Point
}

impl Triangle {
    pub fn tri_area(self) -> f64 {
        // Heron's formula
        let a: f64 = self.a.distance(self.b);
        let b: f64 = self.b.distance(self.c);
        let c: f64 = self.c.distance(self.a);
        let s: f64 = (a+b+c)*0.5;
        
        let area = (s*(s-a)*(s-b)*(s-c)).sqrt();
        return if area.is_nan() {0.0} else {area};
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ {} {} {} }} trig:{:.2}", self.a, self.b, self.c, self.tri_area())
    }
}
impl str::FromStr for Triangle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //{p1,p2,p3}
        let mut points: Vec<String> = vec![String::new();3];
        let mut point_idx = 0;
        let mut is_point = false;
        for c in s.chars() {
            match c {
                '(' => {
                    is_point = true;
                    points[point_idx].push(c);
                }
                ')' => {
                    is_point = false;
                    points[point_idx].push(c);
                    point_idx+=1;
                }
                _ => {
                    if is_point {
                        points[point_idx].push(c);
                    }
                }
            }
        }
        Ok(Self{
            a:points[0].parse()?,
            b:points[1].parse()?,
            c:points[2].parse()?
        })
    }
}

// impl cmp
impl Ord for Triangle {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}
impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return self.tri_area().partial_cmp(&other.tri_area());    
    }
}
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        return self.tri_area() == other.tri_area();
    }
}

/* random */
impl Distribution<Triangle> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Triangle {
        return Triangle{a: rng.gen(), b: rng.gen(), c: rng.gen()};
    }
}