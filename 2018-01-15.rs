//! This error took me one weekend to find out (two days)
//! The error was that the Point units were in point, not in millimeter

pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Here I assumed that the points coming in should be in millimeter
    // but in my use-case, I passed in points already
    pub fn new(x_mm: f64, y_mm: f64) -> Self {
        Self {
            x: x_mm * * 2.834_646_f64, // millimeter to points
            y: y_mm * 2.834_646_f64,
        }
    }
}

fn main() {
    // I could not see from the constructor function that the 
    let point = Point::new(300, 400);
    println!("{}", point.x); // you'd expect "300", but you'd get "850.392"
}
