// ###### CONTROL FLOW IS TO GO OVER IF, WHILE, LOOP, AND MATCH STATEMENTS ######
pub fn if_statement() {
    // Using celsius
    let temp = 25;

    // Normal if, else if, else statements
    if temp > 30 {
        println!("It is really hot outside!")
    } else if temp < 10 {
        println!("It is really cold outside!")
    } else {
        println!("It is just right outside!")
    }


    // One liner if/else statement
    let day = if temp > 20 {"Sunny"} else {"Cloudy"};
    println!("Today is {}!", day);


    // In-line if statements within the print function.
    println!("It is {}", if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"just right"});


    // Nested in-line if statements
    println!("It is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"just right"});
}


pub fn while_and_loop() {
    let mut x = 1;

    // Print numbers as long as they aren't greater than 1000
    while x < 1000 {
        x *= 2;

        // continue skips this value/condition
        // if x == 64 {continue;}
        if x < 1000 {
            println!("x is equal to {}", x);
        } else {
            // Break is killing the loop so it does not continue on
            break;
        }
    }


    // Loop is the equal to while true
    let mut y = 1;
    loop {
        y *= 2;
        println!("y is equal to {}", y);

        // Execute until y becomes equal to 2 to the power of 10.
        // Break is killing the loop so it does not continue on
        if y == 1<<10 {break;}
    }
}


pub fn for_loop() {
    // Initializing x with a range from 1 - 11 (the 11 defines the upper bounds, aka it stops once it hits 11)
    // Aka 11 is excluded in the range when using a ..
    for x in 1..11 {

        // continue skips this value/condition
        if x == 3 {continue;}

        // prints to a value of 7, once it hits 8 it does not get to the print statement.
        // if x == 8 {break;}

        println!("x is equal to {}", x);
    }

    // Here we are getting the values from 30 - 40, and pos is storing the first number as position 1 - 10,
    // and y is storing tha actual values 30 - 40
    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y)
    }
}


pub fn match_statement() {
    let country_code = 249;  // Country codes currently go from 1 - 249

    let country = match country_code {
        1 => "US",
        7 => "Russia",
        44 => "UK",
        46 => "Sweden",
        // 1 - 249
        // Cannot use a .. for a range in a match statement have to use ...
        // Using ... includes 249 in the list of values. It includes the first and last value in the range.
        1...249 => "Unknown",  // 1..99 = 1 - 98, 1...99 = 1 - 99
        // The _ is essentially the catch all for this match statement if the number lies between
        _ => "an Invalid Code"
    };

    println!("The country with the code of {} is {}", country_code, country);
}
