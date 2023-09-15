#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
struct Color(u8, u8, u8); //Tuple Struct

#[allow(dead_code)]
struct Unit; //Unit-Like Struct

impl Rectangle {
    // An associated function (notice no self parameter)
    // Often used for constructors
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // A method (takes a reference to self)
    // Returns the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Another method
    // Checks if the rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//You can have multiple impl blocks for code organization
#[allow(dead_code)]
impl Rectangle {
    fn enlarge(&mut self, scale: u32) {
        self.width *= scale;
        self.height *= scale;
    }
    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
}
impl Rectangle {}
impl Rectangle {}
impl Rectangle {}
impl Rectangle {}
impl Rectangle {}

fn main() {
    // Using an associated function to create an instance of Rectangle
    let rect1 = Rectangle::new(50, 30);

    // Using methods on the Rectangle instance
    println!("The area of rect1 is {} square pixels.", rect1.area());

    let rect2 = Rectangle::new(40, 20);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let mut rect1 = Rectangle::new(50, 30);

    rect1.enlarge(2); // Doubles the size of rect1
    println!("Width: {}, Height: {}", rect1.width, rect1.height); // Output: Width: 100, Height: 60

    //Struct update
    let rect2 = Rectangle {
        height: 50,
        ..rect2
    };

    println!(
        "rect2 => Width: {}, Height: {}",
        rect2.width(),
        rect2.height
    );
}
