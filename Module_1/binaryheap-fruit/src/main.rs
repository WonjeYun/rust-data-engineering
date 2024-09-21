use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Clone, Debug)]
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing Ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad(n:u32) -> BinaryHeap<Fruit> {
    let mut rng = thread_rng();
    let fruits = vec![
        "Apple",
        "Orange",
        "Pear",
        "Peach",
        "Banana",
        "Fig",
        "Fig",
        "Fig",
        "Fig",
    ];
    let mut fruit_salad = BinaryHeap::new();

    let mut figs_count = 0;
    while figs_count < n {
        let fruit = fruits.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn remove_fruit(fruit_salad: &mut BinaryHeap<Fruit>, fruit: Fruit) -> BinaryHeap<Fruit>{
    let mut new_fruit_salad = BinaryHeap::new();
    for f in fruit_salad.iter() {
        if f != &fruit {
            new_fruit_salad.push(f.clone());
        }
    }
    new_fruit_salad
}

fn main() {
    println!("How many figs do you want?");
    let n = read_input().parse::<u32>().unwrap();
    let mut fruit_salad = generate_fruit_salad(n);
    println!("Random Fruit Salad With {:?} Servings of Figs:", n);

    let sorted_fruit = fruit_salad.clone().into_sorted_vec();
    for fruit in sorted_fruit {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }

    println!("Do you want to remove a fruit y/n ?");
    match read_input().as_str() {
        "y" => {
            println!("Which fruit do you want to remove?");
            let unwanted_fruit = read_input();
            let fruit = if unwanted_fruit == "Fig" {
                Fruit::Fig
            } else {
                Fruit::Other(unwanted_fruit.clone())
            };
            println!("Removing {:?} from the fruit salad...", unwanted_fruit);
            fruit_salad = remove_fruit(&mut fruit_salad, fruit);
        }
        _ => (),
    }

    for fruit in fruit_salad.into_sorted_vec() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }
}
