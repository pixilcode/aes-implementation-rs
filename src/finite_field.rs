pub fn ff_add(x: u8, y: u8) -> u8 {
	0
}

pub fn x_time(x: u8) -> u8 {
	0
}

pub fn ff_multiply(x: u8, y: u8) -> u8 {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ff_add_test() {
		assert_eq!(ff_add(0x57, 0x83), 0xd4);
	}

	#[test]
	fn x_time_test() {
		assert_eq!(x_time(0x57), 0xae);
		assert_eq!(x_time(0xae), 0x47);
		assert_eq!(x_time(0x47), 0x8e);
		assert_eq!(x_time(0x8e), 0x07);
	}

	#[test]
	fn ff_multiply_test() {
		assert_eq!(ff_multiply(0x57, 0x13), 0xfe);
	}
}