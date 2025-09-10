// function to convert a string to uppercase
fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

// function to get the length of a string
fn length(s: String) -> usize {
    s.len()
}

// compose function: takes two functions and returns their composition
fn compose<A, B, C>(f: fn(B) -> C, g: fn(A) -> B) -> impl Fn(A) -> C {
    move |x| f(g(x))
}

// runner for function composition example
pub fn run() {
    let names = vec!["Alic", "Bob", "Charlie"];

    // compose functions: first to_uppercase, then length
    let upper_len = compose(length, to_uppercase);

    for name in names {
        println!("{} has length {}", name, upper_len(name));
    }

    println!("----------------------");
}
