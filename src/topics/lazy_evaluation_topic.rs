// runner for lazy evaluation examples
pub fn run() {
    // a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // lazy iterator: map doubles the number, but nothing runs yet
    let doubled = numbers.iter().map(|x| {
        println!("Doubling {}", x); 
        x * 2
    });

    println!("Iterator created, no computation yet!");

    // consume the iterator with collect
    let result: Vec<i32> = doubled.collect();
    println!("Doubled numbers collected: {:?}", result);

    println!();

    // lazy evaluation with filter
    let filtered = numbers
        .iter()
        .filter(|x| {
            println!("Checking if {} is even", x);
            *x % 2 == 0
        })
        .map(|x| x * 10); // still lazy

    println!("Filter iterator created, still lazy");

    // consume with collect
    let filtered_result: Vec<i32> = filtered.collect();
    println!("Filtered & multiplied result: {:?}", filtered_result);

    println!("----------------------");
}
