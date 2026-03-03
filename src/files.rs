// include filesystem library
use std::fs;
use std::path::Path;
use std::process::exit;
use colored::*;

pub fn read_in_file(filename: &String) -> Vec<u8> {
	
	// read in data from file as a vector of bytes (u8 = 1 byte)
	let data: Vec<u8>;

	match fs::read(filename) {
		Ok(content) => {
			data = content;
		},
		Err(error) => {
			println!("\n{} reading file: {}", "Error".red().bold(), error);

			exit(1);
		}
	}

	return data;
}

pub fn write_to_file(filename: &String, content: Vec<u8>) {

	// literally just write to the file
	match fs::write(filename, content) {
		Ok(_) => {},
		Err(_) => println!("Error reading file.")
	}
}

pub fn remove_extension(filename: &String) -> String {

	// create path
	let new_name = Path::new(filename)
		.file_stem()
		.and_then(|s| s.to_str())
		.unwrap_or(filename)
		.to_string();

	return new_name;
}