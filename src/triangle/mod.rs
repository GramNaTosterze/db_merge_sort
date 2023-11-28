use point::Point;
use rand::{prelude::Distribution, distributions::Standard};
use serde_derive::{Serialize, Deserialize};
use std::{fmt};
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
        return (s*(s-a)*(s-b)*(s-c)).sqrt()
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{\n{}\n{}\n{}\n}} trig:{}", self.a, self.b, self.c, self.tri_area())
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