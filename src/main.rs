// Using "use" allows us to call the function as if it resides in this file.
mod rust_data_types;
mod rust_operators;
use rust_operators::operators;

mod memory_management;
mod control_flow;

// THIS FILE HAS TO BE NAMED MAIN AS THE RUST COMPILER NEEDS ONE FILE TO BE MAIN
// SO THIS FILES PURPOSE TO WILL BE TO EXECUTE OUR OTHER FILES.

fn main() {

    println!("###### DATA TYPES/VARIABLES/CONSTANTS IN RUST ######");
    // Since we didn't use "use" for the rust_data_types "mod" we have to call the "path" to the function
    // Function for my test file to learning more about data types
    // rust_data_types::data_types();

    println!("###### OPERATORS IN RUST ######");
    // Function for test file for learning more about Rust Operators
    // operators();

    println!("###### SCOPE AND SHADOWING IN RUST ######");
    // Function for learning about Variable Scopes and Shadowing
    // rust_data_types::scope_and_shadowing();


    println!("###### CONSTANTS IN RUST ######");
    // rust_data_types::constants()


    println!("###### MEMORY MANAGEMENT IN RUST ######");
    // memory_management::memory_management()
    // memory_management::stack_and_heap()


    println!("###### CONTROL FLOW IN RUST ######");
    // control_flow::if_statement()
    // control_flow::while_and_loop()
    // control_flow::for_loop()
    control_flow::match_statement()
}
