pub fn vec_xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {

	// our resultant vector
	let mut c: Vec<u8> = vec![];

	// zip the two vectors together
	for (a_byte, b_byte) in a.iter().cycle().zip(b.iter()) {

		// push new XOR byte to resultant vector
		c.push(*a_byte ^ *b_byte);
	}

	return c;
}

pub fn vec_deep_clear(a: &mut Vec<u8>) {
	for b in a {
		*b = 0b0000_0000;
	}
}

pub fn str_deep_clear(a: &mut String) {
	*a = String::from(" ").repeat(a.chars().count());
}