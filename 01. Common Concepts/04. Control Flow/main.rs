fn main() {
    let number = 3;
    println!("----------------IF EXPRESSION------------------");
    if number < 5 {
        println!("condition was true and number {0} is less than 5", number);
    } else if number > 5 {
        println!("condition was true and number {0} is greater than 5", number);
    }else {
        println!("Neither condition was true");
    }


    let condition = true;
    let number = if condition { 5 } else { 6 }; //Has to be same type evaluated
    println!("The value of number is: {number}");

    println!("----------------INFINITE LOOP------------------");
    //LOOP
    let mut count = 0;
    let loop_result = loop {
        println!("loop: {count}");
        if count == 5 {
            break count * 10;
        }
        count +=1;
    };

    println!("The result is {loop_result}");

    //Loop with labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!("----------------WHILE LOOP------------------");

    //WHILE LOOPS
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("----------------FOR LOOP------------------");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("element: {element}");
    }
}