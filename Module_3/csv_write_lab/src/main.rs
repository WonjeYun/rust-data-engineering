use csv::{Reader, Writer};
use std::{error::Error, io, process};

fn discount_rate(category: &str) -> f64 {
    match category {
        "Electronics" => 0.05,
        "Computers" => 0.10,
        "Photography" => 0.15,
        "Gaming" => 0.02,
        _ => 0.03,
    }
}

fn discount() -> Result<(), Box<dyn Error>> {
    // Open the input CSV file
    // let mut reader = Reader::from_path("products.csv")?;
    let mut reader = Reader::from_reader(io::stdin());

    // Open the output CSV file
    let mut writer = Writer::from_path("data/discounted_products_by_cat.csv")?;

    // Write the header row
    writer.write_record(&[
        "Product Name",
        "Category",
        "Discount Rate",
        "Discounted Price",
    ])?;

    // Iterate through the rows of the input file
    let mut init_total = 0.0;
    let mut discount_total = 0.0;

    for result in reader.records() {
        let record = result?;

        // Extract product name and price
        let name = &record[0];
        let price: f64 = record[1].parse()?;
        init_total += price;

        let category = &record[2];
        let discount_rate = discount_rate(category);
        // Calculate the discounted price
        let discounted_price = price * (1.0 - discount_rate); // 10% discount
        discount_total += discounted_price;

        // Write the product name and discounted price to the output file
        writer.write_record(&[
            name,
            category,
            &format!("{:.2}%", discount_rate * 100.0),
            &format!("{:.2}", discounted_price),
        ])?;
    }

    // Flush the writer to ensure all data is written
    writer.flush()?;
    println!("Initial total: {:.2}", init_total);
    println!("Discounted total: {:.2}", discount_total);
    println!("Total discount: {:.2}", init_total - discount_total);
    println!("Discounted prices have been written to discounted_products.csv");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = discount() {
        eprintln!("error: {}", err);
        process::exit(1);
    }

    Ok(())
}
