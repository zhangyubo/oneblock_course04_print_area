use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
}

// 圆形
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// 三角形
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 正方形
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T>(shape: T) where T: Shape {
    println!("面积: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 3.0 };
    let triangle = Triangle { base: 8.0, height: 3.0 };
    let square = Square { side: 2.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}
