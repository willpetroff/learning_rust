use std::mem;
//Dealing with the stack and the heap

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{ x: 0.0, y: 0.0 }
}

pub fn rust_mem() { // pub declaration needed to run in main
    println!("\n\nMemory Management\n");
    let stack_x = 7;
    let stack_y = 28;
    let heap_z = Box::new(21); // sets heap_z value to pointer value of the "Box" object
    println!("{}, {}, {}", stack_x, stack_y, heap_z);
    println!("stack_x + heap_z = {}", stack_x + *heap_z);
    // The * is a dereferencing opperator, alerting Rust to go to the location of the pointer
    // Without the *, Rust will throw an error when trying to add the int (stack_x) to the pointer (heap_z)
}

pub fn stack_and_heap() {
    println!("\n");
    let p1 = origin(); // stack allocation: holds value
    let p2 = Box::new(origin()); // heap allocation: holds pointer

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    // stack value takes up 16 bytes because it has to store two f64 values
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));
    // heap value takes up 8 bytes because it only stores on f64 pointer value

    let p3 = *p2; // puts p2 value back on the stack
    println!("p3 takes up {} bytes", mem::size_of_val(&p3));
    println!("{}, {}", p3.x, p3.y);
}