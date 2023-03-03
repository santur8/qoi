use std::path::Path;

pub fn file_exists(path: &str) -> bool {
	let p = Path::new(path);
	let exists: bool = p.exists();
	if !exists {
		print!("The file at path {} does not exist. ", path);
		println!("Verify that the file exists in the images directory.");
	}
	exists
}
