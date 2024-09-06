use std::collections::{HashMap, VecDeque};
use std::io::{self, Write};

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn vec_data(vec: &mut Vec<String>) {
    loop {
        let vec_push_pop = get_input("Choose push, pop or end: ");
        match vec_push_pop.as_str() {
            "push" => {
                let push_input = get_input("Enter a string to push: ");
                vec.push(push_input);
            }
            "pop" => match vec.pop() {
                Some(_value) => {
                    println!("Popped last element");
                }
                None => {
                    println!("Vector is empty");
                }
            },
            "end" => {
                println!("Ended with vector result: {:?}", vec);
                break;
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}

fn vecdeque_data(vecdeque: &mut VecDeque<String>) {
    loop {
        let vecdeque_push_pop = get_input("Choose push, pop_front, pop_back or end: ");
        match vecdeque_push_pop.as_str() {
            "push" => {
                let push_input = get_input("Enter a string to push: ");
                vecdeque.push_back(push_input);
            }
            "pop_front" => match vecdeque.pop_front() {
                Some(_value) => {
                    println!("Popped first element");
                }
                None => {
                    println!("VectorDeque is empty");
                }
            },
            "pop_back" => match vecdeque.pop_back() {
                Some(_value) => {
                    println!("Popped last element");
                }
                None => {
                    println!("VectorDeque is empty");
                }
            },
            "end" => {
                println!("Ended with vecdeque result: {:?}", vecdeque);
                break;
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}

fn hashmap_data(hashmap: &mut HashMap<String, String>) {
    loop {
        let hashmap_insert_remove = get_input("Choose insert, remove or end: ");

        match hashmap_insert_remove.as_str() {
            "insert" => {
                let key = get_input("Enter a key: ");
                let value = get_input("Enter a value: ");
                hashmap.insert(key, value);
            }
            "remove" => {
                let remove_key = get_input("Enter a key to remove: ");
                match hashmap.remove(&remove_key) {
                    Some(_value) => {
                        println!("Removed key-value pair");
                    }
                    None => {
                        println!("Key not found");
                    }
                }
            }
            "end" => {
                println!("Ended with hashmap result: {:?}", hashmap);
                break;
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}

fn main() {
    loop {
        let mut vec: Vec<String> = Vec::new();
        let mut vecdeque: VecDeque<String> = VecDeque::new();
        let mut hashmap: HashMap<String, String> = HashMap::new();
        let choice = get_input("Your choice of collection: ");

        match choice.as_str() {
            "vec" => {
                vec_data(&mut vec);
            }
            "vecdeque" => {
                vecdeque_data(&mut vecdeque);
            }
            "hashmap" => {
                hashmap_data(&mut hashmap);
            }
            "end" => {
                println!("End of program");
                break;
            }
            _ => {
                println!("Invalid input");
            }
        }
    }
}
