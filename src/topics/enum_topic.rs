// define an enum for traffic light states
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// implement behavior for the enum
impl TrafficLight {
    pub fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,     // stay red for 60s
            TrafficLight::Yellow => 5,   // yellow for 5s
            TrafficLight::Green => 45,   // green for 45s
        }
    }
}

// define an enum for shapes
pub enum Shape {
    Circle(f64),            // radius
    Rectangle(f64, f64),    // width, height
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
        }
    }
}

// runner for enum examples
pub fn run() {
    // trafficLight example
    let red = TrafficLight::Red;
    println!("Red light duration: {} seconds", red.duration());

    let green = TrafficLight::Green;
    println!("Green light duration: {} seconds", green.duration());

    let yellow = TrafficLight::Yellow;
    println!("Yellow light duration {} seconds", yellow.duration());

    // shape example
    let circle = Shape::Circle(3.0);
    let rect = Shape::Rectangle(4.0, 6.0);

    println!("Area of circle: {:.2}", circle.area());
    println!("Area of rectangle: {:.2}", rect.area());

    println!("----------------------");
}
