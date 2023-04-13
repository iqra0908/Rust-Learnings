use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let mut rng = rand::thread_rng();

    // Create a vector of Person structs with random names and ages
    let mut people = vec![
        Person { name: "Alice".to_string(), age: rng.gen_range(18..=65) },
        Person { name: "Bob".to_string(), age: rng.gen_range(18..=65) },
        Person { name: "Charlie".to_string(), age: rng.gen_range(18..=65) },
        Person { name: "Dave".to_string(), age: rng.gen_range(18..=65) },
        Person { name: "Eve".to_string(), age: rng.gen_range(18..=65) },
    ];

    // Sort the vector by age in ascending order
    people.sort_by(|a, b| a.age.cmp(&b.age));

    // Print out the sorted list of people
    for person in &people {
        println!("{:?}", person);
    }
}
