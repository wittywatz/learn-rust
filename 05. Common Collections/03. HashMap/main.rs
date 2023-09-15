use std::collections::HashMap;

#[allow(unused_variables)]
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); //scores.insert(key, value);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Blue")).or_insert(30); //only insert if the key has no prior value

    //Retreiving a value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //Returns an option enum
    println!("Score: {:?}", score);

    //Iterating over a hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.remove(&team_name); // Removing a value
    let length = scores.len(); //Returns the number of elements the hashmap holds
    let capacity = scores.capacity(); //Returns the number of key-value pairs the hashmap can hold without reallocating.
    println!("length: {:?}", length);

    let is_empty = scores.is_empty(); //Returns true if the hashmap contains no elements.

    //Creates a drainer, which empties the HashMap and produces pairs of keys and values:
    for (k, v) in scores.drain() {
        println!("{}: {}", k, v);
    }
    println!("{}", scores.len());

    // scores.clear(); // Removes all key-value pairs from the hashmap

    scores.insert(String::from("Blue"), 10);
    if scores.contains_key(&team_name) {
        println!("Found a score for team {}", team_name);
    }
    // Iterating with keys()
    for key in scores.keys() {
        println!("key is {}", key);
    }

    // Iterating with values()
    for val in scores.values() {
        println!("{}", val);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.retain(|_, &mut v| v > 10); //Keeps only the elements satisfying the condition

    let cloned_scores = scores.clone(); //Makes a copy of the hashmap ---Memory intensive operation(inefficient)
}
