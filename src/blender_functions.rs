use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use exr::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaveFrame {
    pub frame: usize,
    pub vertices: Vec<Vertex>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaveData {
    pub frames: Vec<WaveFrame>,
}

impl WaveData {
    pub fn new() -> Self {
        WaveData { frames: Vec::new() }
    }

    pub fn add_frame(&mut self, frame: usize, vertices: Vec<Vertex>) {
        self.frames.push(WaveFrame { frame, vertices });
    }

    pub fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let json_data = serde_json::to_string_pretty(&self)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(json_data.as_bytes())?;
        Ok(())
    }
}

pub fn extract_wave_data() {
    let input_folder = "path/to/wave_images/";
    let output_file = "path/to/extracted_wave_data.json";

    match perform_extraction(input_folder, output_file) {
        Ok(_) => println!("Wave data extracted and saved successfully."),
        Err(e) => eprintln!("Error extracting wave data: {}", e),
    }
}

fn perform_extraction(input_folder: &str, output_file: &str) -> io::Result<()> {
    // Create a new WaveData instance
    let mut wave_data = WaveData::new();

    // Get all the image files in the input folder
    let paths = fs::read_dir(input_folder)?;

    for path in paths {
        let path = path?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("exr") {
            let frame_number = path.file_stem().and_then(|s| s.to_str()).and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);

            // Read the EXR image
            let image = read_all_data_from_file(&path).unwrap();
            let size = image.layer_data.inline_size();
            let pixels = image.layer_data.channel("R").unwrap().pixels;

            // Extract vertex data from the image
            let vertices: Vec<Vertex> = pixels
                .iter()
                .enumerate()
                .map(|(i, &brightness)| {
                    let x = (i % size.width as usize) as f32;
                    let y = (i / size.width as usize) as f32;
                    Vertex {
                        x,
                        y,
                        z: brightness, // Use the pixel value directly for the Z value
                    }
                })
                .collect();

            wave_data.add_frame(frame_number, vertices);
        }
    }

    // Save the extracted data to the output file
    wave_data.save_to_file(output_file)
}
