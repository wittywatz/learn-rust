fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

#[allow(dead_code)]
fn five(x:i) -> i32 {
    // Function with return value

    //Put a semicolon at the end of 5
    5
}
