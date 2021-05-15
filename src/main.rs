// Using "use" allows us to call the function as if it resides in this file.
mod rust_operators;
use rust_operators::operators;

mod rust_data_types;

// THIS FILE HAS TO BE NAMED MAIN AS THE RUST COMPILER NEEDS ONE FILE TO BE MAIN
// SO THIS FILES PURPOSE TO WILL BE TO EXECUTE OUR OTHER FILES.

fn main() {

    operators();

    // Since we didn't use "use" for the rust_data_types "mod" we have to call the "path" to the function
    rust_data_types::data_types();

}

