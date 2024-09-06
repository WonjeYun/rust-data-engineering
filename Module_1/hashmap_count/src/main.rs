use std::collections::HashMap;
use std::io;

// fn get_input() -> Vec<i32> {
//     let mut numbers = Vec::new();
//     loop {
//         println!("Enter a number (or end to finish): ");
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();
//         let input = input.trim().to_string();

//         if input == "end" {
//             break;
//         } else {
//             let number: i32 = match input.trim().parse() {
//                 Ok(num) => num,
//                 Err(_) => {
//                     println!("Invalid input. Please enter a number.");
//                     continue;
//                 }
//             };
//             numbers.push(number);
//         }
//     }
//     numbers
// }

// fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
//     let mut frequencies = HashMap::new();

//     for num in numbers {
//         let frequency = frequencies.entry(num).or_insert(0);
//         *frequency += 1;
//     }

//     let mut result = Vec::new();

//     for (num, frequency) in frequencies {
//         result.push((num, frequency));
//     }

//     result
// }

fn get_sentence_input() -> String {
    let mut sentence = String::new();
    println!("Enter a sentence: ");
    io::stdin().read_line(&mut sentence).unwrap();
    sentence
}

fn count_words(sentence: &str) -> HashMap<String, u32> {
    let mut frequencies = HashMap::new();
    let words = sentence.split_whitespace();

    for word in words {
        let frequency = frequencies.entry(word.to_string()).or_insert(0);
        *frequency += 1;
    }

    frequencies
}

fn main() {
    // let numbers = get_input();
    let sentence = get_sentence_input();
    let result = count_words(&sentence);
    println!(
        "The frequency of each word in the sentence is: {:?}",
        result
    );
}
