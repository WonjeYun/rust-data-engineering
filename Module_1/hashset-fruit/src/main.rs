use rand::seq::SliceRandom;
use rand::thread_rng;
// use std::collections::HashSet;
use std::collections::HashMap;

fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    // let mut fruit_set = HashSet::new();
    let mut fruit_set = HashMap::new();
    println!("Enter the number of fruits you want to generate: ");
    let fruit_number: usize = read_input().parse().unwrap();

    println!("Generating {} random fruits...", fruit_number);
    for _ in 0..fruit_number {
        // fruit_set.insert(generate_fruit())
        let fruit = generate_fruit();
        let count = fruit_set.entry(fruit).or_insert(0);
        *count += 1;
    }

    println!("Number of unique fruits generated: {}", fruit_set.len());

    for (fruit, count) in fruit_set.iter() {
        println!("{}: {}", fruit, count);
    }

    // print the unique fruits
    for (fruit, _) in fruit_set.iter() {
        println!("{:?}", fruit);
    }
}
