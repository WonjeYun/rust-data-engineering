/*
Usage:

cargo run -- fruits.csv
or
cargo run -- --fruits "apple, pear"

 */

use clap::Parser;
use fruit_salad_maker::create_fruit_salad;
use std::fs;
use std::io;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Wonje Yun rynelhunter@proton.me",
    about = "Make a Fruit Salad"
)]
struct Opts {
    /// Fruits input as a string of comma separated values
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
}

// Function that converts a csv file to a vector of strings
fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(',').map(|s| s.trim().to_string()).collect()
}
fn display_fruit_salad(fruits: &Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}
fn add_sauce(mut fruits: Vec<String>) -> Vec<String> {
    let mut sauce = String::new();
    println!("Enter the sauce you want to add to your fruit salad:");
    io::stdin()
        .read_line(&mut sauce)
        .expect("Failed to read line");
    fruits.push(sauce);
    fruits
}

fn vec_to_csv(fruits: Vec<String>) {
    let csv_input = fruits.join(",");
    // save the fruit salad to a file
    fs::write("fruit_salad.csv", csv_input).expect("Could not write file");
}

fn main() {
    let opts: Opts = Opts::parse();

    // Use fruits from CSV file or command-line input
    let fruit_list = match opts.csvfile {
        Some(filename) => {
            let fruits = std::fs::read_to_string(filename).expect("Could not read file");
            csv_to_vec(&fruits)
        }
        None => opts
            .fruits
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect(),
    };

    let mut fruit_salad = create_fruit_salad(fruit_list);
    // add sauce to fruit salad
    fruit_salad = add_sauce(fruit_salad);
    // display fruit salad
    display_fruit_salad(&fruit_salad);
    //save fruit salad to a file
    vec_to_csv(fruit_salad);
}
