use std::io::*;
use std::fs::File;

const IMAGE_PATH: &str = "images/";

// Convert the given PNG to QOI
pub fn encode(png: &Result<File>, name: &str) {
	let split: Vec<&str> = name.split('.').collect();
    let full_path: String = format!("{}{}{}", IMAGE_PATH, split[0], ".qoi");
	println!("{}", full_path);
	let qoi: Result<File> = File::create(full_path);
}
