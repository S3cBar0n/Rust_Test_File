// So that we can access this function in our main file we need to put "pub" in front of the function
pub fn operators() {
    // Arithmetic
    let mut a = 2 + 3 * 4; //
    println!("{}", a);

    // The following are the allowed quick operations
    // -= += *= /= %=
    a = a + 1; // ++ and -- quick operations are not supported
    a -= 2; // is equivalent to a = a - 2;

    println!("Remainder of {} / {} = {}", a, 3, (a % 3)); // The last assignment is our math to locate the remainder


    // There is no exponents operator, instead we use the pow function, sampled below
    let a_cubed = i32::pow(a, 3); // This is equivalent to (a * a * a)
    println!("{} cubed is equal to {}", a, a_cubed);


    // powi the i stands for integral, its not a floating point power.
    // powf the f stands for float, aka the power is a float.
    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed is equal to {}, {}^pi is equal to {}", b, b_cubed, b, b_to_pi);


    // Bitwise operators - only for integers
    // 01 OR 10 == 11 == 3_10
    let c = 1 | 2; // Equal to 1 or 2, | = OR, & = AND, ^ = XOR, ! = NOT
    println!("1 | 2 = {}", c);


    // Shift operators
    // Calculating two to the power of 10
    let two_to_power_10 = 1 << 10;
    println!("2^10 = {}", two_to_power_10);


    // Logical operators
    // >, <, >=, <=, ==, !=
    let pi_less_than_4 = std::f64::consts::PI < 4.0;
    println!("Is pi less than 4, {}", pi_less_than_4);

    let x = 5;
    let x_is_5 = x == 5;
    println!("Is x equal to 5, {}", x_is_5);
}










