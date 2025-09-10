// recursive factorial function
pub fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// runner for the recursion example
pub fn run() {
    let num = 5;
    let result = factorial(num);
    println!("Factorial of {} is {}", num, result);

    println!("----------------------");
}
