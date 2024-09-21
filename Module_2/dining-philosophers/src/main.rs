/*
* The dining philosophers problem involves multiple threads needing
* synchronized access to shared resources, risking deadlock.
*
* This code models philosophers as threads and forks as shared Mutex<()>
* wrapped in Arc for thread-safe reference counting.
*
* To prevent deadlock from a "deadly embrace" of waiting for neighboring
* forks, philosophers acquire lower numbered forks first. This breaks
* symmetry and avoids circular waiting.
*
* The Mutexes provide exclusive fork access. The Arc allows sharing forks
* between philosophers.
*
* The simulation prints start time, eating duration, and total time for
* all philosophers. Total time approximately equals philosophers divided
* by forks, as that number can eat concurrently.
*
* Key techniques:
* - Used Mutex<()> to represent exclusive fork access
* - Wrapped in Arc to share Mutexes between threads
* - Numbered philosophers and acquire lower fork first
* - Prints timing metrics for simulation
*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::io;

struct Fork {
    id: u32,
    mutex: Mutex<()>,
}

struct Philosopher {
    id: u32,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}

impl Philosopher {
    fn new(id: u32, name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            left_fork,
            right_fork,
        }
    }

    fn eat(&self) {
        let (first_fork, second_fork) = if self.id % 2 == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        let _first_guard = first_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, first_fork.id);
        let _second_guard = second_fork.mutex.lock().unwrap();
        println!("{} picked up fork {}.", self.name, second_fork.id);

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("{} finished eating.", self.name);

        println!("{} put down fork {}.", self.name, first_fork.id);
        println!("{} put down fork {}.", self.name, second_fork.id);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input
}

fn main() {
    //we only have 4 forks at the table
    println!("Enter number of forks at the table (has to be more than 3)");
    let fork_num = read_input().trim().parse::<u32>().unwrap();

    let forks = (0..fork_num)
        .map(|id| {
            Arc::new(Fork {
                id,
                mutex: Mutex::new(()),
            })
        })
        .collect::<Vec<_>>();

    let mut philosophers = vec![
        ("Jürgen Habermas", 0, 1),
        ("Friedrich Engels", 1, 2),
        ("Karl Marx", 2, 3),
        ("Thomas Piketty", 3, 0),
        ("Michel Foucault", 0, 1),
        ("Socrates", 1, 2),
        ("Plato", 2, 3),
        ("Aristotle", 3, 0),
        ("Pythagoras", 0, 1),
        ("Heraclitus", 1, 2),
        ("Democritus", 2, 3),
        ("Diogenes", 3, 0),
        ("Epicurus", 0, 1),
        ("Zeno of Citium", 1, 2),
        ("Thales of Miletus", 2, 3),
    ]
        .into_iter()
        .enumerate()
        .map(|(id, (name, left, right))| {
            Philosopher::new(
                id as u32,
                name,
                Arc::clone(&forks[left]),
                Arc::clone(&forks[right]),
            )
        })
        .collect::<Vec<_>>();

    let mut total_philosophers: u32 = philosophers.len() as u32;
    println!("There are {} Philosophers at the table, do you want to add more? y/n", &total_philosophers);
    match read_input().trim() {
        "y" => {
            loop {
                println!("Type a philosopher to add or exit");
                let new_philosopher_nm = read_input().trim().to_string();
                if new_philosopher_nm == "exit" {
                    break;
                } else {
                    println!("Type the left fork idx");
                    let new_left: usize = read_input().trim().parse::<usize>().unwrap();

                    println!("Type the right fork idx");
                    let new_right: usize = read_input().trim().parse::<usize>().unwrap();

                    let new_philosopher = Philosopher::new(
                        total_philosophers,
                        &new_philosopher_nm,
                        Arc::clone(&forks[new_left]),
                        Arc::clone(&forks[new_right]),
                    );
                    philosophers.push(new_philosopher);
                    total_philosophers += 1;
                }
            }
        },
        "n" => (),
        _ => println!("Invalid input. Please type y/n")
    }

    println!("Dining Philosophers Problem:  {} Philosophers, {} Forks...Yikes!!", philosophers.len(), fork_num);

    let start = Instant::now();

    let handles = philosophers
        .into_iter()
        .map(|philosopher| {
            thread::spawn(move || {
                philosopher.eat();
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}