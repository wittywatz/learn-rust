fn main() {
    println!("----------------OWNERSHIP------------------");
    let s1 = String::from("hello"); // s1 owns the string "hello"
    let s2 = s1; // ownership is transferred to s2
                 // println!("{}", s1);           // This would cause an error as s1 no longer owns the string
    println!("{}", s2);

    println!("----------------BORROWING------------------");
    let s3 = &s2; // s3 borrows a reference to s2's data
    let s4 = &s2; // s2 can be borrowed immutably multiple times
    println!("s3: {}, s4: {}", s3, s4);

    println!("----------------MUTABLE BORROWING------------------");
    let mut s5 = String::from("world");
    let s6 = &mut s5; // s6 mutably borrows s5
                      // let s7 = &mut s5;             // This would be an error as you can't have multiple mutable borrows
    println!("s6: {}", s6);

    // s5 cannot be accessed directly after it's been mutably borrowed until the mutable borrow goes out of scope
    // println!("s5: {}", s5);       // This would be an error

    s6.push_str(", hello!"); // modify the string via mutable reference
    println!("s6 after modification: {}", s6);
{
    
}
    println!("------------------OWNERSHIP WITH FUNCTIONS------------------");

    let s1 = String::from("hello");
    let s2 = takes_and_gives_back(s1);
    // println!("s1 after modification: {}", s1); // This would be an error ass1 has been moved
    let s3 = &s2; // Immutable reference
    print_string(s3);

    let mut s4 = String::from("world");
    modify_string(&mut s4);
    print_string(&s4);

    assert_eq!(s2, "hello");
    assert_eq!(s4, "world, hello!");

    println!("------------------SLICE TYPE------------------");

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}

// This function takes ownership of the string and then gives it back
fn takes_and_gives_back(s: String) -> String {
    println!("Inside takes_and_gives_back: {}", s);
    s
}

// This function borrows an immutable reference to the string and prints it
fn print_string(s: &String) {
    println!("Printed from function: {}", s);
}

// This function borrows a mutable reference to the string and modifies it
fn modify_string(s: &mut String) {
    s.push_str(", hello!");
}
