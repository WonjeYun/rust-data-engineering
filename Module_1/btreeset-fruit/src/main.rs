use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeMap;
// use std::collections::BTreeSet;

// fn read_input() -> String {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).unwrap();
//     input.trim().to_string()
// }

// // function to remove a fruit from the BTreeSet
// fn remove_fruit(fruit_set: &mut BTreeSet<&str>) {
//     println!("Which fruit would you like to remove?");
//     let remove_fruit = read_input();
//     match fruit_set.remove(remove_fruit.as_str()) {
//         true => println!("{} removed", remove_fruit),
//         false => println!("{} not found", remove_fruit),
//     }
// }

fn main() {
    let fruits = vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
        "honeydew",
    ];
    let amounts = [1, 3, 5, 7, 9];

    let mut rng = thread_rng();

    for amount in amounts.iter() {
        // let mut fruit_set = BTreeSet::new();
        let mut fruit_set = BTreeMap::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        // // add fruits to the BTreeSet until the amount is reached
        // for fruit in shuffled_fruits {
        //     fruit_set.insert(fruit);
        //     if fruit_set.len() >= *amount {
        //         break;
        //     }
        // }

        for fruit in shuffled_fruits.iter().take(*amount) {
            let count = fruit_set.entry(fruit).or_insert(0);
            *count += 1;
        }

        // // ask the user if they would like to remove a fruit
        // println!("Would you like to remove a fruit? y/n");
        // let remove_bool = read_input();
        // match remove_bool.as_str() {
        //     "y" => remove_fruit(&mut fruit_set),
        //     _ => (),
        // }

        // // print the fruit set in reverse order
        // for fruit in fruit_set.iter().rev() {
        //     println!("{}", fruit);
        // }
        println!("{}: {:?}", amount, fruit_set);
    }
}
