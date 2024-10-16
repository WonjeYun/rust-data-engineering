//command-line tool that reads a CSV file and prints the contents of the file as a DataFrame
use clap::Parser;
use polars::prelude::*;
const CSV_FILE: &str = "src/data/global-life-expt-2022.csv";

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A command-line tool that reads a CSV file and prints the contents of the file as a DataFrame",
    after_help = "Example: cargo run -- print --rows 3"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Print {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long, default_value = "10")]
        rows: usize,
    },
    Describe {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Schema {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Shape {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Sort {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long, default_value = "2020")]
        column: String,
        #[clap(long, default_value = "10")]
        rows: usize,
        #[clap(long, default_value = "T")]
        order: String,
    },
    Filter {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long, default_value = "20")]
        column: String,
        #[clap(long, default_value = "10")]
        rows: usize,
        #[clap(long, default_value = "gt")]
        method: String,
        #[clap(long, default_value = "0.0")]
        value: f32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Print { path, rows }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.head(Some(rows)));
        }
        Some(Commands::Describe { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df);
        }
        Some(Commands::Schema { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.schema());
        }
        Some(Commands::Shape { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.shape());
        }
        Some(Commands::Sort {
            path,
            column,
            rows,
            order,
        }) => {
            let df = polarsdf::read_csv(&path);
            let country_column_name = "Country Name";
            //select the country column and the year string passed in and return a new dataframe
            let vs = df.select_series([country_column_name, &column]).unwrap();
            //convert the Vec<Series> to a DataFrame
            let df2 = DataFrame::new(vs).unwrap();
            //drop any rows with null values and return a new dataframe
            let mut df2: DataFrame = df2.drop_nulls::<String>(None).unwrap();
            //sort the dataframe by the year column and by order passed in
            match order.as_str() {
                "T" => {
                    df2 = df2
                        .sort(
                            [&column],
                            SortMultipleOptions::new().with_order_descending(true),
                        )
                        .unwrap()
                }

                "F" => {
                    df2 = df2
                        .sort(
                            [&column],
                            SortMultipleOptions::new().with_order_descending(false),
                        )
                        .unwrap()
                }

                _ => println!("Expected T/F, got {:?}", order),
            }

            //print the first "rows" of the dataframe
            println!("{:?}", df2.head(Some(rows)));
        }
        Some(Commands::Filter {
            path,
            column,
            rows,
            method,
            value,
        }) => {
            let df = polarsdf::read_csv(&path);
            let country_column_name = "Country Name";
            //select the country column and the year string passed in and return a new dataframe
            let vs = df.select_series([country_column_name, &column]).unwrap();
            //convert the Vec<Series> to a DataFrame
            let mut df2 = DataFrame::new(vs).unwrap();
            //sort the dataframe by the year column and by order passed in
            match method.as_str() {
                "gt" => {
                    let mask = df
                        .column(&column)
                        .expect("no matching columns")
                        .gt(value)
                        .expect("not valid value");
                    df2 = df2.filter(&mask).expect("not valid value");
                }

                "ge" => {
                    let mask = df
                        .column(&column)
                        .expect("no matching columns")
                        .gt_eq(value)
                        .expect("not valid value");
                    df2 = df2.filter(&mask).expect("not valid value");
                }

                "lt" => {
                    let mask = df
                        .column(&column)
                        .expect("no matching columns")
                        .lt(value)
                        .expect("not valid value");
                    df2 = df2.filter(&mask).expect("not valid value");
                }

                "le" => {
                    let mask = df
                        .column(&column)
                        .expect("no matching columns")
                        .lt_eq(value)
                        .expect("not valid value");
                    df2 = df2.filter(&mask).expect("not valid value");
                }

                "notnull" => {
                    let mask = df
                        .column(&column)
                        .expect("no matching columns")
                        .is_not_null();
                    df2 = df2.filter(&mask).expect("not valid value");
                }

                _ => println!("Expected gt/ge/lt/le/notnull, got {:?}", method),
            }
            //print the first "rows" of the dataframe
            println!("{:?}", df2.head(Some(rows)));
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
