mod input;
mod files;
mod encrypt;
use colored::*;

fn main() {

	println!("\nWould you like to {} or {}?", "encrypt".blue().bold(), "decrypt".blue().bold());
	print!("  {}", "> ".blue().bold());

	let choice = input::input();

	if choice.chars().nth(0).expect("") == 'e' {
		encrypt_routine();
	} else {
		decrypt_routine();
	}
}

fn encrypt_routine() {
	println!("\nWhat {} do you wish to encrypt?", "file".blue().bold());
	print!("  {}", "> ".blue().bold());

	// collect filename information
	let filename = input::input();

	// variables regarding choosing a key, and whether or not the user remembers said key
	let mut key: String = String::new();
	let mut confirmed_key: String;
	let mut unconfirmed_key = true;

	// as long as the user cannot remember their key, we will not let them encrypt
	while unconfirmed_key {
		println!("\nWhat {} do you wish to use to encrypt? (don't forget it!)", "key".blue().bold());
		print!("  {}", "> ".blue().bold());

		key = input::input();

		println!("\nConfirm key:");
		print!("  {}", "> ".blue().bold());

		confirmed_key = input::input();

		// determine if they remembered the key
		if key == confirmed_key {
			unconfirmed_key = false;
		} else {
			println!("\nKeys do {} match.", "not".red().bold());
		}
	}

	// read in from the file as binary bytes (u8 = 1 byte = 8 bits)
	let mut data_bytes: Vec<u8> = files::read_in_file(&filename);
	let mut key_bytes: Vec<u8> = key.as_bytes().to_vec();

	let encrypted = encrypt::vec_xor(&key_bytes, &data_bytes);

	let new_filename = filename + ".crypt";

	files::write_to_file(&new_filename, encrypted);

	println!("\nFile encryption was a {}!", "success".green().bold());
	
	// overwrite key & data with 0's so nobody's the wiser
	encrypt::vec_deep_clear(&mut key_bytes);
	encrypt::vec_deep_clear(&mut data_bytes);
	encrypt::str_deep_clear(&mut key);
}

fn decrypt_routine() {
	println!("\nWhat {} do you wish to decrypt?", "file".blue().bold());
	print!("  {}", "> ".blue().bold());

	// collect filename information
	let filename = input::input();

	println!("\nWhat {} do you wish to use to encrypt? (don't forget it!)", "key".blue().bold());
	print!("  {}", "> ".blue().bold());

	let mut key = input::input();

	let mut key_bytes: Vec<u8> = key.as_bytes().to_vec();
	let mut data_bytes: Vec<u8> = files::read_in_file(&filename);

	let decrypted = encrypt::vec_xor(&key_bytes, &data_bytes);

	let real_filename = files::remove_extension(&filename);

	files::write_to_file(&real_filename, decrypted);

	println!("\nFile decryption was {} (hopefully)!", "successful".bold().green());

	encrypt::vec_deep_clear(&mut key_bytes);
	encrypt::vec_deep_clear(&mut data_bytes);
	encrypt::str_deep_clear(&mut key);
}