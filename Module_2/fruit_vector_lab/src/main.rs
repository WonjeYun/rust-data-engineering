use std::collections::HashMap;

fn display_fruits(fruit_vec: &Vec<&str>) {
    for fruit in fruit_vec.iter() {
        println!("{}", fruit);
    }
    println!("-----------------");
}

fn remove_fruit(fruit_vec: &mut Vec<&str>, fruit: &str) {
    let index = fruit_vec.iter().position(|&x| x == fruit).unwrap();
    fruit_vec.remove(index);
    println!("Removed fruit: {}", fruit);
    display_fruits(fruit_vec);
}

fn sort_fruits(fruit_vec: &mut Vec<&str>) {
    fruit_vec.sort();
    display_fruits(fruit_vec);
}

fn count_fruits(fruit_vec: &mut Vec<&str>) {
    let mut fruit_count = HashMap::new();
    for fruit in fruit_vec.iter() {
        let count = fruit_count.entry(fruit).or_insert(0);
        *count += 1;
    }

    for (fruit, count) in fruit_count.iter() {
        println!("{}: {}", fruit, count);
    }
}

fn main() {
    let mut fruit_vec = vec!["apple", "banana", "cherry", "pineapple", "mango", "kiwi"];
    display_fruits(&fruit_vec);

    fruit_vec.push("date");
    display_fruits(&fruit_vec);

    fruit_vec.pop();
    display_fruits(&fruit_vec);

    remove_fruit(&mut fruit_vec, "banana");

    sort_fruits(&mut fruit_vec);

    count_fruits(&mut fruit_vec);
}
