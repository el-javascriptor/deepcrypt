// include filesystem library
use std::fs;
use std::path::Path;

pub fn read_in_file(filename: &String) -> Vec<u8> {
	
	// read in data from file as a vector of bytes (u8 = 1 byte)
	let data: Vec<u8> = fs::read(filename).expect("Error reading file.");

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