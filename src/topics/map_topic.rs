use std::collections::HashMap;

// function to map names to ages
pub fn map_fun(names: Vec<String>, ages: Vec<i32>) -> HashMap<String, i32> {
    let mut students = HashMap::new();

    for i in 0..names.len() {
        students.insert(names[i].clone(), ages[i]); 
    }

    students
}

// runner for map example
pub fn run() {
    let names: Vec<String> = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
        String::from("Diana"),
        String::from("Ethan"),
    ];

    let ages: Vec<i32> = vec![20, 21, 19, 22, 20];

    let students = map_fun(names, ages);

    for (name, age) in &students {
        println!("{} is {} years old", name, age);
    }

    println!("----------------------");
}
