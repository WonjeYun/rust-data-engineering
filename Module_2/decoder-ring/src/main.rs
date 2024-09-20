
/*
Attempts to statistically decode a Caesar cipher.
Here's an example of how to use it:

This is a shift 16 message: "Off to the bunker. Every person for themselves"
"Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc"

cargo run -- --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --guess

Here is an example of it in action:

Shift: 0, Score: 7.538945
Shift: 1, Score: 10.078025
Shift: 2, Score: 20.755177
Shift: 3, Score: 11.284368
Shift: 4, Score: 7.5232525
Shift: 5, Score: 23.558884
Shift: 6, Score: 21.086407
Shift: 7, Score: 9.926911
Shift: 8, Score: 5.5866623
Shift: 9, Score: 15.310673
Shift: 10, Score: 17.950832
Shift: 11, Score: 21.200842
Shift: 12, Score: 23.447578
Shift: 13, Score: 14.035946
Shift: 14, Score: 13.314641
Shift: 15, Score: 2.055822
Shift: 16, Score: 40.54977
Shift: 17, Score: 15.98934
Shift: 18, Score: 18.178614
Shift: 19, Score: 8.523561
Shift: 20, Score: 9.371011
Shift: 21, Score: 12.385875
Shift: 22, Score: 8.159725
Shift: 23, Score: 10.439689
Shift: 24, Score: 17.104122
Shift: 25, Score: 14.300304
Best shift: 16 (out of 26), score: 40.54977
Decrypted message: Off to the bunker. Every person for themselves

*/

use clap::Parser;
use decoder_ring::print_stats_analysis;
use std::fs::File;
use std::io::Read;

/// CLI tool to reverse engineer a Caesar cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // The message to decrypt
    #[arg(short, long)]
    message: Option<String>,

    // The path to the message
    #[arg(short, long)]
    path: Option<String>,

    // statistical information about the message
    #[arg(short, long)]
    stats: bool,

    // guess the shift
    #[arg(short, long)]
    guess: bool,
}

fn read_file(path: String) -> String{
    let mut file = File::open(path).expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file to string");
    contents
}

// run it
fn main() {
    let args = Args::parse();
    let mut encrypt = String::new();

    match (&args.message, &args.path) {
        (Some(_), Some(_)) => {
            eprintln!("Error: You cannot specify both --message and --path. Choose one.");
            std::process::exit(1);
        }
        (None, None) => {
            eprintln!("Error: You must specify either --message or --path.");
            std::process::exit(1);
        }
        _ => {
            // Proceed if exactly one of --message or --path is provided
            if let Some(message) = args.message {
                encrypt = message;
            } else if let Some(path) = args.path {
                // Read the message from the file path
                encrypt = read_file(path);
                // Here you would add file reading logic
            }
        }
    }

    //stats
    if args.stats {
        //print_stats_analysis(&args.message);
        print_stats_analysis(&encrypt);
    }
    //guess
    if args.guess {
        // let (depth, best_shift, decrypted, max_score) = decoder_ring::guess_shift(&args.message, 26);
        let (depth, best_shift, decrypted, max_score) = decoder_ring::guess_shift(&encrypt, 26);
        println!(
            "Best shift: {} (out of {}), score: {}",
            best_shift, depth, max_score
        );
        println!("Decrypted message: {}", decrypted);
    }
}
