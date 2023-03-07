use std::env;
use std::io::*;
use std::fs::File;
use std::path::Path;

mod encode;
mod decode;

const IMAGE_PATH: &str = "images/";

fn main() {
	println!("");
	// verify that we have 3 arguments: executable, operation, and file
	let argc: usize = env::args().len();
	let argv: Vec<String> = env::args().collect();
    let full_path: String = format!("{}{}", IMAGE_PATH, &argv[2]);
	if argc != 3 || !file_exists(&full_path) {
		println!("Invalid arguments supplied.\nArguments should follow the form \"cargo run <file operation> <file>\"");
		std::process::exit(-1);
	}

	if &argv[1] == "encode" {
		println!("encoding");
		encode::encode(&argv[2]);
	} else if &argv[1] == "decode" {
		println!("decoding");
	} else {
		println!("Invalid file operation. Choose from the following:\nencode\ndecode");
		std::process::exit(-1);
	}
}

/* Verify that the image to encode exists */ 
fn file_exists(path: &str) -> bool {
    let p = Path::new(&path);
    let exists: bool = p.exists();
    if !exists {
        print!("The file at path {} does not exist. ", path);
        println!("Verify that the file exists in the images directory.");
    }
    exists
}
