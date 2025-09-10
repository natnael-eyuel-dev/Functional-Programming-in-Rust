// a pure function: same input => same output, no side effects
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// another pure function: square of a number
pub fn square(x: i32) -> i32 {
    x * x
}

// runner for pure functions
pub fn run() {
    let x = 5;
    let y = 3;

    let sum1 = add(x, y);
    let sum2 = add(x, y);    // calling again to show same output

    println!("add({}, {}) = {}", x, y, sum1);
    println!("Calling add({}, {}) again = {}", x, y, sum2);

    let sq = square(x);
    println!("square({}) = {}", x, sq);

    println!("----------------------");
}

