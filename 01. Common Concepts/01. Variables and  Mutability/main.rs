fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("3 hours in seconds is {THREE_HOURS_IN_SECONDS}");

    //Variables
    let x = 5;
    println!("The value of x is: {x}");
    // x = 6; // Uncomment this line to see the behavior
    println!("The value of x is: {x}");

    // Mutability
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    //Shadowing
    {
        let x = x * 10;
        println!("The value of x in the inner scope is: {x}");
        {
            
        }
    }

    println!("The value of x is still: {x}");
}
