#[allow(unused_variables)]
fn main() {
    let v: Vec<i32> = Vec::new(); // Creates an empty vector
    let v = vec![1, 2, 3]; // Uses the `vec!` macro for initialization

    //Accessing elements of a vector
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // Direct indexing
    let third: Option<&i32> = v.get(2); // Using the `get` method which returns an Option

    //Append to a vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(3);
    v.push(3);

    println!("{:?}", v);

    v.pop(); // Removes the last element

    for i in &v {
        println!("{}", i);
    }

    let len = v.len();
    let capacity = v.capacity();

    println!("v has capacity of {} bytes", capacity);
    println!("v has a length of {}", len);

    // v.clear(); //Clearing the vector

    // v.is_empty() //Check if vector is empty

    if let Some(position) = v.iter().position(|&x| x == 3) {
        println!("Found 3 at position {}", position);
    }

    let v = vec![1, 2, 3, 4, 5];
    let evens: Vec<_> = v.iter().filter(|&x| x % 2 == 0).collect();
    println!("{:?}", v);
    println!("evens: {:?}", evens);

    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    v1.extend(v2);
    println!("new v1 ---> {:?}", v1);

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let extended: Vec<_> = a1.iter().chain(a2.iter()).collect();
    println!("{:?}", extended);

    //Sorting
    let mut v = vec![4, 1, 3, 2];
    v.sort(); //sort_unstable is faster
    println!("sorted {:?}", v);
    v.reverse();
    println!("reversed {:?}", v);
}
