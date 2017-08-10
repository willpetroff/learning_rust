pub fn if_statement()
{
    println!("\n\nControl Flow\n\n");
    let temp = 45;

    if temp > 30
    {
        if temp >= 35 // Nested if statement
        {
            println!("It's {} degrees out. That's really, really hot.", temp);
        }
        else
        {
            println!("It's {} degrees out. That's really hot.", temp);
        }

    }
    else if temp < 10
    {
        println!("It's {} degrees out. That's really cold.", temp);
    }
    else
    {
        println!("It's {} degrees out. That's not so hot or so cold.", temp);
    }
    //println!("\n");

    // "if" statement is an expression in rust, meaning you can use it in an assignment argument
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("Today looks like it will be {}\n", day);
}

pub fn while_and_loops()
{
    let mut x = 1;

    while x < 1000
    {
        x *= 2;

        // The "continue" call tells rust to go back to the top of the loop if the coonditions are met
        if x > 1000 {continue;}

        println!("x = {}", x);
    }
    println!("\n");
    
    let mut y = 1;
    loop // basically a "while true" call; loops till you break out of it
    {
        y += y;
        println!("y = {}", y);

        if y == 1048576 {break;} // the "break" call exits out of a loop
    }
    println!("\n");

}