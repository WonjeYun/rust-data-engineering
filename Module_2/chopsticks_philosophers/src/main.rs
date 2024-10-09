use std::thread;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use rayon::prelude::*;

struct Chopsticks {
    id: u32,
    mutex: Mutex<()>,
}

struct Philosopher {
    id: u32,
    name: String,
    short_chopsticks: Arc<Chopsticks>,
    long_chopsticks: Arc<Chopsticks>,
}

impl Philosopher {
    fn new(id: u32, name: &str, short_chopsticks: Arc<Chopsticks>, long_chopsticks: Arc<Chopsticks>) -> Philosopher {
        Philosopher {
            id,
            name: name.to_string(),
            short_chopsticks,
            long_chopsticks
        }
    }

    fn eat(&self) {
        let (first, second) = if self.id % 2 == 0 {
            (&self.short_chopsticks, &self.long_chopsticks)
        } else {
            (&self.long_chopsticks, &self.short_chopsticks)
        };

        let _first_chopstick = first.mutex.lock().unwrap();
        println!("{} picked up chopstick {}.", self.name, first.id);
        let _second_chopstick = second.mutex.lock().unwrap();
        println!("{} picked up chopstick {}.", self.name, second.id);

        println!("Philosopher {} ({}) is eating.", self.id, self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("Philosopher {} ({}) is done eating.", self.id, self.name);

        println!("{} put down chopstick {}.", self.name, first.id);
        println!("{} put down chopstick {}.", self.name, second.id);
    }
}

fn main() {
    // Threads pass messages
    let (tx, rx) = mpsc::channel();

    // Mutex protects shared state
    let counter = Arc::new(Mutex::new(0));

    // Rayon parallelizes
    (0..10).into_par_iter().for_each(|x| {
        let mut count = counter.lock().unwrap();
        *count += 1;
        tx.send(x).unwrap();
    });

    // Receive and print results
    for _ in 0..10 {
        println!("Received: {}", rx.recv().unwrap());
    }

    println!("Final count: {}", *counter.lock().unwrap());

    // Create chopsticks
    // let num_chopsticks = Arc::new(5u32);
    // let table = Arc::new((0..*num_chopsticks).map(|_| Mutex::new(())).collect::<Vec<_>>());
    let chopsticks = (0..4)
        .map(|id| {
            Arc::new(Chopsticks {
                id,
                mutex: Mutex::new(()),
            })
        })
        .collect::<Vec<_>>();

    // Create philosophers
    let philosophers = vec![
        ("Socrates", 0, 1),
        ("Plato", 1, 2),
        ("Aristotle", 1, 0),
        ("Thales", 2, 1),
        ("Pythagoras", 1, 0),
    ]
        .into_iter()
        .enumerate()
        .map(|(id, (name, long, short))| {
            Philosopher::new(
                id as u32,
                name,
                Arc::clone(&chopsticks[long]),
                Arc::clone(&chopsticks[short]),
            )
        })
        .collect::<Vec<_>>();

    let start = Instant::now();
    // Spawn philosopher threads
    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                p.eat();
            })
        }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total time: {:?}", start.elapsed());
}
