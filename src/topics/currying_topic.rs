// normal function taking two arguments
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// curried version using closures
fn curried_add(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

// runner for closures example
pub fn run() {
    // using normal function
    let sum = add(5, 3);
    println!("add(5, 3) = {}", sum);

    // using curried function
    let add_five = curried_add(5); // partially applied
    let result = add_five(3);      // complete application
    println!("curried_add(5)(3) = {}", result);

    // another example
    let add_ten = curried_add(10);
    println!("curried_add(10)(7) = {}", add_ten(7));

    println!("----------------------");
}
