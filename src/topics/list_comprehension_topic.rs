// runner for list comprehension examples
pub fn run() {
    // original list of numbers
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // example 1: Square each number 
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);

    // example 2: Filter even numbers and multiply by 10
    let even_times_ten: Vec<i32> = numbers
        .iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 10)
        .collect();
    println!("Even numbers * 10: {:?}", even_times_ten);

    // example 3: Combine map and filter like list comprehension
    let result: Vec<String> = numbers
        .iter()
        .filter(|x| *x % 2 != 0) 
        .map(|x| format!("Number {}", x))
        .collect();
    println!("Odd numbers formatted: {:?}", result);

    println!("----------------------");
}
