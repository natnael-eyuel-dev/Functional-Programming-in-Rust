// runner for Sum (reducing collection of values)
pub fn run() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // using iterator and sum()
    let total: i32 = numbers.iter().sum();
    println!("Sum of all numbers: {}", total);

    // sum of squares
    let sum_of_squares: i32 = numbers.iter().map(|x| x * x).sum();
    println!("Sum of squares: {}", sum_of_squares);

    // sum of only even numbers
    let sum_even: i32 = numbers.iter().filter(|x| *x % 2 == 0).sum();
    println!("Sum of even numbers: {}", sum_even);

    println!("----------------------");
}
