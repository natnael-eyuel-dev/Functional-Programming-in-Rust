// runner for switch/case example
pub fn run() {
    let number = 3;

    // match in Rust works like switch, but safer & more powerful
    match number {
        1 => println!("Number is One"),
        2 => println!("Number is Two"),
        3 => println!("Number is Three"),
        4 | 5 => println!("Number is Four or Five"), 
        _ => println!("Number is something else"),   
    }

    // example: day of week
    let day = "Mon";

    match day {
        "Mon" => println!("Start of the work week"),
        "Fri" => println!("Almost weekend"),
        "Sat" | "Sun" => println!("Weekend!"),
        _ => println!("Midweek day"),
    }

    println!("----------------------");
}
