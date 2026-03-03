
// include standard input / output
use std::io;
use std::io::Write;
use colored::*;

pub fn input() -> String {

	// our default input carrot thing
	let carrot: String = format!("  {}", "> ".blue().bold());

	print!("{carrot}");

	// flush console output
	io::stdout().flush().expect("Failed to flush console.");
	
	// create a string to store input
	let mut str_in = String::new();

	// read cmd line input into string
	let successful = io::stdin().read_line(&mut str_in);

	// test to see if command line input was successful
	match successful {

		// if successful, naught to worry about
		Ok(_n_bytes) => {},

		// if unsuccessful, return a blank string
		Err(_err) => {str_in = String::new()}
	}

	// trim whitespace, return string
	return str_in.trim().to_string();
}