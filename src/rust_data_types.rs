// ###### THIS SECTION IS FOR LEARNING ABOUT THE BASIC DATA TYPES AND SCOPE FEATURES IN RUST ######
// This is importing our libraries
use std::mem;

// This is one way on how you can create a global variable for this specific rust file
// Unlike regular variables that can only be used within their founding scope,
// this can be used in any scope of this file.
// It is also important to note that there cannot be any variables declared below that have the same
// name as the constants we are declaring here.

// The value below is essentially just pasted into where we need it.
// Const is better for using predefined variables/values such as PI.
const CONSTANT_VALUE:u8 = 42;  // This has no fixed address, memory safety is not compromised.

// This is a safe static variable.
static Z:i32 = 123;  // This does have a fixed address

// Having a static mutable can create memory issues with multiple threads reading and writing to the variable.
// This is unsafe and be avoided at all costs, unless you have some advanced memory management skills
static mut Z_MUTABLE:i32 = 999;


// This is our function for learning about Data Types specifically
// So that we can access this function in our main file we need to put "pub" in front of the function
pub fn data_types() {
    // unsigned number is 0 - 255
    // signed number is positive or negative and called with i (-127 - 128)
    let a:u8 = 123;  // u8 = unsigned 8bits

    // The curly braces are us telling Rust we want to insert a variable there.
    // After our curly braces we do a comma and then put in the name of our variable.
    // At run time this will replace the curley braces with the value of our variable a.
    println!("a = {}", a);


    // Variables are immutable which means you cannot change the value
    // So the below will not work
    // a = 456;  // Immutable

    // To have a Variable you can change (aka mutable)
    // You need to call it with let mut a:u8 (for example)
    let mut b:i8 = 0;  // Mutable
    println!("b = {}", b);

    b = 42;
    println!("b = {}", b);


    // You do not have to declare your variable name such as b:i8, Rust can try to figure it out
    // So using let v = 0; will also be acceptable

    let mut c = 123456789;  // 32-bit signed integer
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    c = -1;
    println!("c = {} after reassignment", c);


    // isize and usize, it gets you the size of the memory address that your system is running on
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);


    // The following string has a size of 4 bytes
    let d = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    // The following float (double precision) has a size of 8 bytes or 64 bits, f64 data type
    // Unless specifically defined this will default to 64 bit which will make it 8 bytes by default.
    let e = 2.5;
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));


    // Booleans
    let f = false;
    println!("f = {}, size = {} bytes", f, mem::size_of_val(&f));

    let g = 4 > 0;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}


// This function is to help with learning about Scope and Shadowing in the scope_of_variables function
pub fn scope_and_shadowing() {
    let a = 123;

    // This is an additional scope inside of the function.
    // The Variable we called above inside of this functions scope will work in the scope below.
    // The variable here will only work inside the scope, you cannot call this variable out side of
    // the curly braces.
    {
        let b = 456;
        println!("Inside the scope, b's value is {}", b);

        // We can redeclare variables inside of these sub-scopes (you can't within the same scope)
        // Doing this is known as shadowing the original variable.
        let a = 777;

        println!("Inside the scope, a's is equal to {}", a);
    }

    println!("Outside the scope, a's is equal to {}", a);
}

// This function is for learning about scope of variables and functions.
// Once a variable has been used in a different function and the program continues to another
// that variable in the other function is destroyed and unusable.
// A scope is actually defined by whats inside of the curly braces of a function.
pub fn scope_of_variables() {
    // This is us trying to call the "a" variable from the very top function above,
    // it will fail since its out of the functions scope.
    // This is commented out so it wont fail at compile time.
    // println!("a is {}", a)
}

pub fn constants() {
    println!("{} is the value of our CONSTANT_VALUE constant", CONSTANT_VALUE);

    println!("{} is the value of our Z constant", Z);

    // By default Rust will not allow you to use a mutable variable, but if we call an unsafe block
    // it will let us run the below print statement
    // Calling this unsafe block is us saying we promise that we are working carefully with the variable.
    unsafe {
        println!("{} is the value of our Z_MUTABLE constant", Z_MUTABLE);

        Z_MUTABLE = 555;
        println!("{} is the value of our Z_MUTABLE constant after changing the value", Z_MUTABLE);
    }

}