/* Extend this example */
use std::time::Instant;
use rayon::prelude::*;

fn main() {

    for i in 3..10 {
        let data: Vec<i32> = (0..i).collect();

        let start = Instant::now();
        let parallel_sum: i32 = data.par_iter() // Specify the type
            .map(|x| x * x)
            .sum();

        println!("Total parallel time: {:?}", start.elapsed());
        println!("Parallel sum: {}", parallel_sum);

        let seq_start = Instant::now();
        let mut seq_sum: i32 = 0;
        for num in data {
            seq_sum += num * num;
        }
        println!("Total sequential time: {:?}", seq_start.elapsed());
        println!("Sequential sum: {}", seq_sum);
    }
}