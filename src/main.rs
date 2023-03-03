use std::env;

mod encode;
mod decode;

fn main() {
	println!("");
	// verify that we have 3 arguments: executable, operation, and file
	let argc: usize = env::args().len();
	let argv: Vec<String> = env::args().collect();
	if argc != 3 || !encode::file_exists(&argv[2]) {
		println!("Invalid arguments supplied.\nArguments should follow the form \"cargo run <file operation> <file>\"");
		std::process::exit(-1);
	}

	let op: i32;
	if &argv[1] == "encode" {
		println!("encoding");
	} else if &argv[1] == "decode" {
		println!("decoding");
	}
}
