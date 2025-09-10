// define an enum for different shapes
enum Shape {
    Circle(f64),              // radius
    Rectangle(f64, f64),      // width, height
    Triangle(f64, f64, f64),  // sides a, b, c
}

// runner for pattern matching examples
pub fn run() {
    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in shapes {
        match shape {
            Shape::Circle(r) => {
                println!("Circle with radius {} → area {:.2}", r, std::f64::consts::PI * r * r);
            }
            Shape::Rectangle(w, h) => {
                println!("Rectangle {}x{} → area {:.2}", w, h, w * h);
            }
            Shape::Triangle(a, b, c) if a + b > c && a + c > b && b + c > a => {
                let s = (a + b + c) / 2.0;
                let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
                println!("Valid Triangle ({}, {}, {}) → area {:.2}", a, b, c, area);
            }
            Shape::Triangle(_, _, _) => {
                println!("Invalid Triangle!");
            }
        }
    }

    // pattern matching with tuples
    let point = (0, -5);
    match point {
        (0, y) => println!("Point is on Y axis at y={}", y),
        (x, 0) => println!("Point is on X axis at x={}", x),
        (x, y) => println!("Point is at ({}, {})", x, y),
    }

    println!("----------------------");
}
