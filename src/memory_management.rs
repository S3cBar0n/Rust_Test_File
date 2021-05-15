// ###### THIS IS FOR GOING OVER MEMORY MANAGEMENT IN RUST ######
use std::mem;

// We are declaring a struct for later for a custom data type.
struct Point {
    x: f64,
    y: f64
}


pub fn memory_management() {
    // This statement allocates a chunk of RAM where the value below is stored.
    // Anytime we call the below variable we are reading our RAM to get the value.
    // The chunk of RAM that is being allocated for these variables is called the "Stack", which is for
    // short term storage, aka stored here only for the life time of a particular function.
    // Short term storage known as the "Stack" is fast and limited in space.
    let x = 5;  // i32 variable (4 bytes of memory)
    println!("{} is our x Variable stored on Stack memory", x);


    // Long term memory needs to be stored in a "Heap".
    // Using Box::new() lets us store our variable in the "Heap" for long term storage.
    // The Value of y is allocated on the opposite side of RAM from the "Stack".
    // So y is stored on the "Stack", but what we mean by this is that it is only a pointer containing
    // a memory address telling us that the value of y is stored on the "Heap" at this memory address.
    // AKA when we are calling "y" we are calling the memory address, and to get the value we want
    // we need to put a * in front of the variable name when calling it (*y).
    // The * tells us to follow the memory address and find out the value of what the variable is
    // pointing to.
    let y = Box::new(10);
    println!("{} is our y Variable stored on Heap memory", *y);

    // This is a similar concept to the above.
    // This is essentially storing the individual values of 1,2,3 on the "Heap"
    // and the value is printed with {:?} is used to debug and print the value.
    // No * needed in this case.
    // There is more than one way to print the value of a vec.
    let z = vec![1, 2, 3];
    println!("{:?} is our z Variable stored on Heap memory", z);
}

// We are using this to create data for the below demo.
// We are assigning the values 0.0 to the x & y data struct we created at the start of the file.
// So basically you assign a variable to the value of this function and it will give you the points
// x and y with the value of 0.0
fn origin() -> Point {
    Point{x: 0.0, y:0.0}
}


pub fn stack_and_heap() {
    // This is a demo of Stack vs Heap

    // p1 is on the Stack memory
    let p1 = origin();
    // p2 is on the Heap memory
    let p2 = Box::new(origin());

    // Since the p1 variable's value is on Stack memory it stores the actual value on Stack,
    // which adds the total of all the bytes of the data aka 8 + 8.
    println!("p1 uses {} bytes in Stack Memory", mem::size_of_val(&p1));
    // Since the p2 variable's value is on Heap memory it is getting the total bytes of the memory address
    // that is stored on the Stack which only takes up a single byte so 8.
    println!("p2 uses {} bytes in Stack Memory", mem::size_of_val(&p2));

    // Here we are assigning the actual value of p2 which is located in the heap to p3 with a *.
    let p3 = *p2;
    // Here we are printing the value of x, and if we swaap p3.x with a y it will print the value of y.
    println!("p2 uses {} bytes in Heap Memory", p3.x);
}
