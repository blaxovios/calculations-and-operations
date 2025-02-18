use polars::prelude::*;
use std::env;


pub fn join_polars_dataframes() -> Result<(), Box<dyn std::error::Error>> {
    // Read data from CSV files.
    env::set_var("RUST_BACKTRACE", "full");

    let dataframe1 = LazyCsvReader::new("first.csv").finish()?;
    let dataframe2 = LazyCsvReader::new("second.csv").finish()?;
    let dataframe3 = LazyCsvReader::new("third.csv").finish()?;
    let dataframe3 = dataframe3.collect()?;
    let join_key = "aid";

    let first_joined_df = dataframe1.clone().inner_join(dataframe2.clone(), join_key, join_key).collect()?;
    println!("Joined dataframe no. 1.");

    let second_joined_df = first_joined_df.clone().lazy().inner_join(dataframe3.clone().lazy(), join_key, join_key).collect()?;
    println!("Joined dataframe no. 2.");

    let mut final_df = second_joined_df.clone();
    println!("Collected dataframes.");

    let mut file = std::fs::File::create("data/exports/output_data.parquet").unwrap();
    ParquetWriter::new(&mut file).finish(&mut final_df).unwrap();
    println!("Created parquet!");

    Ok(())
}