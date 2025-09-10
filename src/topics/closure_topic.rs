// runner for closure examples
pub fn run() {
    // simple closure assigned to a variable
    let add = |a: i32, b: i32| a + b;
    println!("add(5, 3) = {}", add(5, 3));

    // closure that captures outer variable
    let factor = 10;
    let multiply = |x: i32| x * factor;
    println!("multiply(6) with captured factor = {}", multiply(6));

    // closure with no parameters
    let greet = || println!("Hello from closure!");
    greet();

    // closure used in iterator
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled numbers using closure in map: {:?}", doubled);

    println!("----------------------");
}
