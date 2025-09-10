// a function that add two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// a function that multiplies two numbers
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// runner 
pub fn run() {
    // partial application using closures
    let add_five = |x: i32| add(5, x); // pre-filling first argument
    let multiply_by_two = |x: i32| multiply(2, x); // pre-filling first argument

    // compose functions: multiply first, then add
    let multiply_then_add_five = |x: i32| add_five(multiply_by_two(x));

    let num = 4;
    println!("add_five({}) = {}", num, add_five(num));
    println!("multiply_by_two({}) = {}", num, multiply_by_two(num));
    println!("multiply_then_add_five({}) = {}", num, multiply_then_add_five(num));

    println!("----------------------");
}
