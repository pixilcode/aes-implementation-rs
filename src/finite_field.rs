//! Calculations done with finite fields, which are represented as bytes

pub struct FiniteField(u8);

impl std::ops::Add for FiniteField {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
		FiniteField(ff_add(self.0, rhs.0))
    }
}

impl std::ops::Mul for FiniteField {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
		FiniteField(ff_multiply(self.0, rhs.0))
    }
}

impl From<u8> for FiniteField {
    fn from(value: u8) -> Self {
        FiniteField(value)
    }
}

impl Into<u8> for FiniteField {
	fn into(self) -> u8 {
		self.0
	}
}

/// The irreducible finite field associated with AES encryption
const M_X: u8 = 0x1b;

/// Add two finite fields together
fn ff_add(x: u8, y: u8) -> u8 {
	x ^ y
}

/// Multiply a finite field by `x`
fn x_time(x: u8) -> u8 {
	if (x & 0b10000000) == 0 {
		x << 1
	} else {
		(x << 1) ^ M_X
	}
}

/// Multiply two finite fields together
fn ff_multiply(x: u8, y: u8) -> u8 {
	let mut current_field = x;
	let mut remaining_bits = y;
	let mut result = 0;

	loop {
		if remaining_bits == 0 {
			return result;
		}

		if (remaining_bits & 1) == 1 {
			result = ff_add(result, current_field);
		}

		remaining_bits >>= 1;
		current_field = x_time(current_field);
	}
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