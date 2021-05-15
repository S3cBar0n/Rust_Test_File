// This is importing our libraries
use std::mem;

// So that we can access this function in our main file we need to put "pub" in front of the function
pub fn data_types() {
    // unsigned number is 0 - 255
    // signed number is positive or negative and called with i (-127 - 128)
    let a:u8 = 123; // u8 = unsigned 8bits

    println!("a = {}", a);


    // Variables are immutable which means you cannot change the value
    // So the below will not work
    // a = 456; // Immutable

    // To have a Variable you can change (aka mutable)
    // You need to call it with let mut a:u8 (for example)
    let mut b:i8 = 0; // Mutable
    println!("b = {}", b);

    b = 42;
    println!("b = {}", b);


    // You do not have to declare your variable name such as b:i8, Rust can try to figure it out
    // So using let v = 0; will also be acceptable

    let mut c = 123456789; // 32-bit signed integer
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