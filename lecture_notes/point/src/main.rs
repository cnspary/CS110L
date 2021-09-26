use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

pub trait ComputerNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputerNorm for Option<u32> {} // use default compute_norm

impl ComputerNorm for Point {
    fn compute_norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl ComputerNorm for Vec<f64> {
    fn compute_norm(&self) -> f64 {
        self.iter().map(|x| {x * x}).sum::<f64>().sqrt()
    }
}

impl Add for Point {
    type Output = Self;  // an associated type 
    fn add(self, other: Self) -> Self {
        Point::new (self.x + other.x, self.y + other.y)
    }
}

fn main() {
    let the_origin = Point::new(0.0, 0.0);
    let mut p = the_origin;

    println!("p: {:?}, the_origin: {:?}", p, the_origin);
    println!("are they equal? => {}", p == the_origin);
    
    p.x += 10.0;
    println!("p: {:?}, the_origin: {:?}", p, the_origin);
    println!("are they equal? => {}", p == the_origin);

    let x: Option<u32> = Some(1);
    println!("Option<u32>(Some) type's compute_norm: {}", x.compute_norm());
    let x: Option<u32> = None;
    println!("Option<u32>(None) type's compute_norm: {}", x.compute_norm());
    let x: Point = Point::new(4.0, 3.0);
    println!("Point type's compute_norm: {}", x.compute_norm());
    println!("{:?} + {:?} = {:?}", p, x, p + x);
    let x: Vec<f64> = vec![4.0, 3.0];
    println!("Vec<f64> type's compute_norm: {}", x.compute_norm()) ;

}
