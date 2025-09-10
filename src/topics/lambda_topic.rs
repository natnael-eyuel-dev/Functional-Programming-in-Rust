// runner for the lambda examples
pub fn run() {
    // a simple lambda (closure) that adds two numbers
    let add = |a: i32, b: i32| a + b;

    let result = add(5, 7);
    println!("add(5, 7) using lambda = {}", result);

    // a lambda that captures an outer variable
    let factor = 10;
    let multiply = |x: i32| x * factor;

    let m_result = multiply(6);
    println!("multiply(6) with captured factor (10) = {}", m_result);

    // a lambda for squaring numbers
    let square = |n: i32| n * n;

    println!("square(4) using lambda = {}", square(4));

    println!("----------------------");
}
