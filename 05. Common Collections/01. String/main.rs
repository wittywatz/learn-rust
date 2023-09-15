#[allow(unused_variables)]
fn main() {
    let s1 = String::new(); // Creates an empty `String`
    let s2 = "Hello, world!".to_string(); // Converts a string literal to a `String`
    let s3: &str = &s2; // Creates a `&str` from a `String`

    // Using format Macro
    let name = "Alice";
    let age = 30;
    let introduction = format!("My name is {} and I am {} years old.", name, age);

    // Updating a String
    let mut s = String::from("Hello");
    s.push(' '); // Push character
    s.push_str("world!"); // Push a string slice
    println!("{}", s);

    let len = s.len(); // Returns length of string in bytes, not characters.
    let capacity = s.capacity(); // Returns capacity in bytes
    println!("s has capacity of {} bytes", capacity);
    println!("s has a length of {}", len);

    let s = String::from("           Rust is fantastic!             ".trim());
    assert!(s.contains("Rust"));
    assert!(s.starts_with("Rust"));
    assert!(s.ends_with("!"));

    for word in s.split_whitespace() {
        println!("{}", word);
    }

    let upper = s.to_uppercase();
    let lower = s.to_lowercase();
    println!("{} to uppercase is {}", s, upper);
    println!("{} to lowercase is {}", s, lower);

    //Bytes
    for b in "hello".bytes() {
        println!("{} byte maps to  ----> {}", b, b as char);
    }

    //Reading lines from a String
    for line in "first\nsecond\nthird".lines() {
        println!("{}", line);
    }

    //Convert string to another type
    let my_num: u32 = "42".parse().expect("Not a number!");

    //drain method
    let mut s = String::from("foobar");
    let drained: String = s.drain(2..4).collect();
    println!("{}", drained); // "ob"
    println!("{}", s); //foar

    //insert  and insert_str methods
    let mut s = String::from("foo");
    s.insert(2, 'o'); // "fooo"
    s.insert_str(3, "bar"); // foobaro
    println!("{}", s); //foobaro

    //Retain method
    let mut s = String::from("hello world");
    s.retain(|c| c != ' ');
    println!("{}", s); // "helloworld"
}
