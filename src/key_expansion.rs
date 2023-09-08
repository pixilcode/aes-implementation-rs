use crate::constant::S_BOX;

fn sub_word(word: u32) -> u32 {
	let word_bytes = word.to_be_bytes();

	let sub_bytes: Vec<u8> = word_bytes.into_iter()
		.map(|byte| {
			let row = (byte >> 4) & 0b1111;
			let column = byte & 0b1111;
			S_BOX[row as usize][column as usize]
		})
		.collect();

	u32::from_be_bytes(sub_bytes.try_into().unwrap())
}

fn rot_word(word: u32) -> u32 {
	word.rotate_left(8)
}

pub fn key_expansion(key: &[u8; 16]) -> [u32; 44] {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sub_word_test() {
		assert_eq!(sub_word(0x00102030), 0x63cab704);
		assert_eq!(sub_word(0x40506070), 0x0953d051);
		assert_eq!(sub_word(0x8090a0b0), 0xcd60e0e7);
		assert_eq!(sub_word(0xc0d0e0f0), 0xba70e18c);
	}

	#[test]
	fn rot_word_test() {
		assert_eq!(rot_word(0x09cf4f3c), 0xcf4f3c09);
		assert_eq!(rot_word(0x2a6c7605), 0x6c76052a);
	}

	#[test]
	fn key_expansion_test() {
		let key: [u8; 16] = [
			0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
		];

		let expanded: [u32; 44] = [
			0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c,
            0xa0fafe17, 0x88542cb1, 0x23a33939, 0x2a6c7605,
            0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f,
            0x3d80477d, 0x4716fe3e, 0x1e237e44, 0x6d7a883b,
            0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00,
            0xd4d1c6f8, 0x7c839d87, 0xcaf2b8bc, 0x11f915bc,
            0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
            0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f,
            0xead27321, 0xb58dbad2, 0x312bf560, 0x7f8d292f,
            0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e,
            0xd014f9a8, 0xc9ee2589, 0xe13f0cc8, 0xb6630ca6,
		];

		assert_eq!(key_expansion(&key), expanded);
	}
}