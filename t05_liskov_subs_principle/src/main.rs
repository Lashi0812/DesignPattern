trait Shape {
    fn calculate_area(&self) -> i32;
}

struct Square {
    side: i32,
}

impl Shape for Square {
    fn calculate_area(&self) -> i32 {
        self.side * self.side
    }
}

struct Circle {
    radius: i32,
}

impl Shape for Circle {
    fn calculate_area(&self) -> i32 {
        self.radius * self.radius
    }
}

fn print_area(shape: &impl Shape) {
    println!("Area: {}", shape.calculate_area());
}

fn main() {
    let square = Square { side: 10 };
    let circle = Circle { radius: 5 };
    print_area(&square);
    print_area(&circle);
}
