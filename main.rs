use std::mem; // Imports standard library: memory module for use
mod rust_memory;
mod rust_control_flow;

const THE_ANSWER:u8 = 42; // You must give the variable a type; it has no fixed address

static ANSWER_HALVED:u8 = 21; // Static declarations have fixed addresses

static mut RAND_NUM:i32 = -244; // Can have mutable constants, but creates unsafe memory management

fn main() {
    rust_primative_data_types();
    rust_operators();
    rust_scope_shadow();
    rust_constants();
    rust_memory::rust_mem();
    rust_memory::stack_and_heap();
    rust_control_flow::if_statement();
    rust_control_flow::while_and_loops();
}

fn rust_primative_data_types() {
    println!("Primative Data Types and Assignments\n");
    let a:u8 = 123; 
    // The "let" binds the variable "a" to the int "123"
    // The "u" indicates that this is an unsigned number; "u"= unsigned integer, "i" = integer
    // The "8" after the "u" indicates the number of bits used for the assignment
    // A single byte (8 bits) allocated for an unsigned number can range from 0 to 255
    // A single byte allocated for a signed number ranges from -127 to 128
    println!("a = {}", a);
    // An attempt to write "a = 456;" causes an error; the variable "a" was set as an immutable object
    // Rust variables are immutable by default
    
    let mut b:i8 = 17; // The "mut" declaration sets the variable as a mutable object
    println!("mutable object b = {}", b);
    b = b + 12;
    println!("now mutable object b = {}", b);

    let mut c = 123456789; //Rust will automatically try to assign object types and sizes
    println!("c = {}, size of c = {} bytes", c, mem::size_of_val(&c)); // "c" = signed 32-bit integer
    // Uses "mem::size_of_val(&"variable") function and returns size of object in bytes (std::mem)
    c = -1; // Change mutable object to show "c" is signed
    println! ("mutable, signed object c = {}", c);
    // numbers can range from i/u-8 to i/u-64

    let z:isize = 123; // isize/usize sets size of variable to max size of OS
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}\nz takes up {} bytes\nthis is a {}-bit OS",
         z, size_of_z, size_of_z*8);

    let d:char = 'x'; // stores a single character in object "d"
    println!("d = {}\nd takes up {} bytes", d, mem::size_of_val(&d));
    // Object "d" char takes up 4 bytes, largest unicode size available

    let e = 2.5; // Floats are double-percision values
    println!("e = {}\ne takes up {} bytes", e, mem::size_of_val(&e));
    // By default, double-percision values take up 8 bytes
    let e:f32 = 2.5; // Sets "e" as a 32-bit floating point number
    println!("e = {}\ne takes up {} bytes", e, mem::size_of_val(&e));

    let f = true;
    let g = false;
    println!("f = {} and g = {}", f, g);
    println!("the size of f is {} byte\nthe size of g is {} byte", mem::size_of_val(&f), mem::size_of_val(&g));
    let h = 4>0;
    println!("h is {}", h);
}

fn rust_operators() {
    //math
    println!("\n\nOperators\n");
    let mut num_1 = 2+3*4; // Follows order of precedence
    num_1 = num_1 + 1; // Rust doesn't support ++/-- operators for up/down movement
    num_1 -= 1; // Rust does alow (operator)= assignments
    println!("{}", num_1);

    // Going to a power
    let num_2 = 3;
    let num_2_cubed = i32::pow(num_2, 3); // Rust has no power operator; requires function
    println!("num_2 = {}\nnum_2_cubed = {}", num_2, num_2_cubed);

    let num_3 = 2.5;
    let num_3_cubed = f64::powi(num_3, 3); // Must use "powi" function to note use of integer exponent
    let num_3_to_pi = f64::powf(num_3, std::f64::consts::PI); // Must use "powf" function to note use of floating exponent
    // Rust has constant numbers stored in the standard library. The above shows "std::f64::consts::(constant)" the syntax.
    // Note that the (constant) is written in ALL CAPS
    println!("num_3 = {}\nnum_3_cubed = {}\nnum_3_to_pi = {}", num_3, num_3_cubed, num_3_to_pi);

    // Bitwise operators
    let num_4 = 1 | 2; // Bitwise: 01 or 10 = 11 (3 in decimal)
    println!("1|2 = {}", num_4);

    let num_5 = 1 << 10; // takes 1 (2) and shifts the bit 10 spaces to the left resulting in 10000000000 (1024)
    // >> shifts to the right
    println!("num_5 = {}", num_5); 

    // Logical Operators
    let pi_less_4 = std::f64::consts::PI < 4.0; // Must match number types
    // Rust uses the normal logical operators
    println!("Is Pi less than 4? {}", pi_less_4);
}

fn rust_scope_shadow() {
    println!("\n\nScope and Shadowing\n");

    // Braces create scope
    let a:u8 = 123;
    let a:u8 = 124; // Declaring a variable twice overides the first declaration
    {
        let b = 125;
        println!("\nInner Scope\n");
        println!("Inside the scope, b = {}",b);
        println!("a = {}", a); // The variable "a" is accessible from the outer scope
        let a:u8 = 248; // Setting value of "a" inside scope, shadowing and overriding the variable in the outer scope
        println!("a = {}", a);
        println!("\nInner Scope\n");
    }
    println!("Outside the scope, a = {}", a);
    
}

fn rust_constants() {
    println!("\n\nConstants and Global Variables\n");
    println!("{}", THE_ANSWER);
    println!("{}", ANSWER_HALVED);
    unsafe {
        // "unsafe" code block lets everyone know you're working with a mutable constant
        println!("RAND_NUM = {}", RAND_NUM);
        RAND_NUM = 523;
        println!("RAND_NUM = {}", RAND_NUM);
   }
}