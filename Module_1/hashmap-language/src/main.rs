use std::collections::HashMap;
use std::io;

fn get_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);

    languages
}

fn add_language(languages: &mut HashMap<String, i32>) {
    println!("Enter the name of the language:");
    let language = get_input();

    println!("Enter the year the language was created:");
    let year = get_input().parse().expect("Please enter a number");

    languages.insert(language, year);
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    // Subtract the creation year from 2024 to get the number of years active.
    for year in years_active.values_mut() {
        *year = 2024 - *year;
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();

    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1; // weight between 1 and 100
        weights.insert(language.to_string(), weight);
    }

    weights
}

fn sort_hash_by_val(hash: &HashMap<String, i32>) -> Vec<(&String, &i32)> {
    let mut vec: Vec<_> = hash.iter().collect();
    vec.sort_by(|a, b| a.1.cmp(b.1));
    vec
}

fn main() {
    let mut languages = init_languages();

    loop {
        println!("Would you like to add a language? (y/n)");
        let input = get_input();
        match input.as_str() {
            "n" => break,
            "y" => add_language(&mut languages),
            _ => println!("Invalid input, please enter 'y' or 'n'"),
        }
    }

    let weights = calculate_weights(&mut languages);
    let weights = sort_hash_by_val(&weights);
    println!("Language weighing from 1-100 by age (1 is newest and 100 is oldest):");

    for (language, weight) in &weights {
        println!("{}: {}", language, weight);
    }
}
