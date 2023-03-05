use std::path::Path;

const IMAGE_PATH: &str = "images/";

pub fn file_exists(path: &str) -> bool {
	let full_path = format!("{}{}", IMAGE_PATH, path);
	let p = Path::new(&full_path);
	let exists: bool = p.exists();
	if !exists {
		print!("The file at path {} does not exist. ", full_path);
		println!("Verify that the file exists in the images directory.");
	}
	exists
}
