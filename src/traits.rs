fn main() {
    distance_method();
}

#[derive(Debug, Clone, Copy)]
struct Point { x: i32, y: i32 }

impl Point {
    fn distance(&self, p2: Point) -> f64 {
        (((self.x - p2.x) as f64).powi(2) 
            + ((self.y - p2.y) as f64).powi(2)).sqrt()
    }
}

fn distance_method() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 2, y: 0 };
    let p3 = Point { x: 0, y: 2 };

    println!("{:?}", p1.distance(p2));
    println!("{:?}", p1.distance(p3));
    println!("{:?}", p2.distance(p3));
}