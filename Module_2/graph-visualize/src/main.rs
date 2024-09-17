extern crate rasciigraph;

use rasciigraph::{plot, plot_many, Config};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // let cities = vec!["Lisbon", "Madrid", "Paris", "Berlin", "Copenhagen", "Stockholm", "Moscow"];
    // let distances_travelled = vec![0.0, 502.56, 1053.36, 2187.27, 2636.42, 3117.23, 4606.35];

    let file = File::open("travel_data.txt")?;
    let reader = BufReader::new(file);

    let mut cities = Vec::new();
    let mut distances_travelled = Vec::new();

    for line in reader.lines() {
        let data_line = line?;
        let parts: Vec<&str> = data_line.split(',').collect();
        if parts.len() == 2 {
            cities.push(parts[0].to_string());
            if let Ok(distance) = parts[1].parse::<f64>() {
                distances_travelled.push(distance);
            }
        }
    }

    println!("{}", cities.join(" > "));

    // // Line graph
    // println!(
    //     "{}",
    //     plot(
    //         distances_travelled.into_iter().map(|d| d as f64).collect(),
    //         Config::default()
    //             .with_offset(10)
    //             .with_height(10)
    //             .with_caption("Travelled distances (km)".to_string())
    //     )
    // );

    // // Multple graph
    // let reverse_distance = distances_travelled.iter().map(|&d| 4606.0 - d).collect();
    // let distance_matrix = vec![distances_travelled, reverse_distance];
    // println!("Multiple Graph:");
    // println!(
    //     "{}",
    //     plot_many(
    //         distance_matrix,
    //         Config::default()
    //             .with_offset(10)
    //             .with_height(10)
    //             .with_caption("Travelled distances (km)".to_string())
    //     )
    // );

    Ok(())
}
