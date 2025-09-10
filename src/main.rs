mod topics {
    pub mod functional_programming;
    pub mod builder_pattern_topic;
    pub mod recursion_topic;
    pub mod pure_function_topic;
    pub mod lambda_topic;
    pub mod enum_topic;
    pub mod switch_topic;
    pub mod pattern_matching_topic;
    pub mod list_comprehension_topic;
    pub mod lazy_evaluation_topic;
    pub mod closure_topic;
    pub mod partial_composition_topic;
    pub mod currying_topic;
    pub mod sum_topic;
    pub mod function_composition_topic;
    pub mod map_topic;
}


fn main() {
    println!("--- Functional Programming Main Topic ---");
    topics::functional_programming::run();

    println!("\n--- Running all examples ---");

    println!("Builder Pattern Example:");
    topics::builder_pattern_topic::run();

    println!("Recursion Example:");
    topics::recursion_topic::run();

    println!("Pure Function Example:");
    topics::pure_function_topic::run();

    println!("Lambda Example:");
    topics::lambda_topic::run();

    println!("Enum Example:");
    topics::enum_topic::run();

    println!("Switch/Case Example:");
    topics::switch_topic::run();

    println!("Pattern Matching Example:");
    topics::pattern_matching_topic::run();

    println!("List Comprehension Example:");
    topics::list_comprehension_topic::run();

    println!("Lazy Evaluation Example:");
    topics::lazy_evaluation_topic::run();

    println!("Closure Example:");
    topics::closure_topic::run();

    println!("Partial Composition Example:");
    topics::partial_composition_topic::run();

    println!("Currying Example:");
    topics::currying_topic::run();

    println!("Sum Example:");
    topics::sum_topic::run();

    println!("Function Composition Example:");
    topics::function_composition_topic::run();

    println!("Map Example:");
    topics::map_topic::run();
}

