/// This is the entry point of the application. It will include the top-level modules.

mod data_dev;
mod blender_utils;


fn main() {
    // let _ = blender_utils::extract_wave_data();
    let _ = data_dev::dataframes::join_polars_dataframes();
}