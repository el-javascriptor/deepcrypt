
// include standard input / output
use std::io;

// we need this to be able to flush the stdout
use std::io::Write;

/// Reads input from the stdin, trims whitespace, and returns.
/// 
/// **Side Effects**:
/// - May return error if failed to flush.
/// - Returns blank string if read is unsuccessful.
pub fn input() -> String {

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