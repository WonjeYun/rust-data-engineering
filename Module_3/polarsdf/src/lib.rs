// utilities for working with polars dataframes
//
use polars::prelude::*;

//read in a csv file
pub fn read_csv(path: &str) -> DataFrame {
    let polars_df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into())).expect("Unable to read file");
    polars_df.finish().expect("Unable to read file")
}

//print "n" rows of a dataframe
pub fn print_df(df: &DataFrame, n: usize) {
    println!("{:?}", df.head(Some(n)));
}

//print the schema of a dataframe
pub fn print_schema(df: &DataFrame) {
    println!("{:?}", df.schema());
}

//print the shape of a dataframe
pub fn print_shape(df: &DataFrame) {
    println!("{:?}", df.shape());
}
