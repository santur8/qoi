use std::io::*;
use std::fs;
use std::fs::File;

const IMAGE_PATH: &str = "images/";

// Convert the given PNG to QOI
pub fn encode(name: &str) {
	// create .qoi file in images directory
	let split: Vec<&str> = name.split('.').collect();
    let qoi_path: String = format!("{}{}{}", IMAGE_PATH, split[0], ".qoi");
	let qoi: File = File::create(qoi_path).unwrap();

	// open png file
	let full_path = format!("{}{}", IMAGE_PATH, name);
	let png: File = File::open(&full_path).unwrap();

	let mut png_buf: Vec<u8> = fs::read(full_path).expect("Unable to open png");
}
