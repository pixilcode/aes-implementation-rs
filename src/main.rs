mod cipher;
mod constant;
mod debug;
mod finite_field;
mod inverse_cipher;
mod key_expansion;

fn main() {
    autograder::run();
    appendix_c::run();
}

mod autograder {
    use crate::{
        cipher::cipher, constant::KeyType, debug::hex_array_to_string,
        inverse_cipher::inverse_cipher, key_expansion::key_expansion,
    };

    pub fn run() {
        let key_128: [u8; 16] = [
            0x39, 0x9a, 0x58, 0xd3, 0x45, 0x5a, 0xa7, 0xf5, 0x48, 0x95, 0x5e, 0x7a, 0x83, 0x0f,
            0x7d, 0x78,
        ];
        decrypt_128(&key_128);
        encrypt_128(&key_128);

        let key_192: [u8; 24] = [
            0x39, 0x9a, 0x58, 0xd3, 0x45, 0x5a, 0xa7, 0xf5, 0x48, 0x95, 0x5e, 0x7a, 0x83, 0x0f,
            0x7d, 0x78, 0xcd, 0x4f, 0x94, 0x3a, 0xa1, 0x07, 0x74, 0xfa,
        ];
        decrypt_192(&key_192);
        encrypt_192(&key_192);

        let key_256: [u8; 32] = [
            0x39, 0x9a, 0x58, 0xd3, 0x45, 0x5a, 0xa7, 0xf5, 0x48, 0x95, 0x5e, 0x7a, 0x83, 0x0f,
            0x7d, 0x78, 0xcd, 0x4f, 0x94, 0x3a, 0xa1, 0x07, 0x74, 0xfa, 0x02, 0xd2, 0x13, 0xbe,
            0x1e, 0xfa, 0xd1, 0x7a,
        ];
        decrypt_256(&key_256);
        encrypt_256(&key_256);
    }

    fn decrypt_128(key: &[u8; 16]) {
        let message = [
            0xda, 0xdb, 0x7a, 0xd6, 0x26, 0x68, 0xeb, 0xf6, 0x2a, 0x4e, 0xcd, 0xad, 0x52, 0x3d,
            0xf3, 0x66,
        ];
        let key_type = KeyType::Aes128;

        let expanded_key = key_expansion(key, key_type);
        let plaintext = inverse_cipher(message, &expanded_key, key_type);
        let plaintext = hex_array_to_string(&plaintext);

        println!("decrypt_128 = {plaintext}");
    }

    fn encrypt_128(key: &[u8; 16]) {
        let message = [
            0x1b, 0x80, 0xf9, 0xd9, 0x3c, 0x29, 0xdb, 0x27, 0x8c, 0x1a, 0x4c, 0xdd, 0x0f, 0xe8,
            0x14, 0xff,
        ];
        let key_type = KeyType::Aes128;

        let expanded_key = key_expansion(key, key_type);
        let plaintext = cipher(message, &expanded_key, key_type);
        let plaintext = hex_array_to_string(&plaintext);

        println!("encrypt_128 = {plaintext}");
    }

    fn decrypt_192(key: &[u8; 24]) {
        let message = [
            0x84, 0x99, 0x77, 0xd9, 0x18, 0xaa, 0x8f, 0xe7, 0x35, 0x33, 0x31, 0xa0, 0xa8, 0x16,
            0xbf, 0x7d,
        ];
        let key_type = KeyType::Aes192;

        let expanded_key = key_expansion(key, key_type);
        let plaintext = inverse_cipher(message, &expanded_key, key_type);
        let plaintext = hex_array_to_string(&plaintext);

        println!("decrypt_192 = {plaintext}");
    }

    fn encrypt_192(key: &[u8; 24]) {
        let message = [
            0x04, 0x75, 0x3d, 0x96, 0x3f, 0x69, 0x09, 0x4a, 0xbd, 0xcb, 0x6f, 0xcd, 0x3b, 0xf9,
            0x3a, 0xf4,
        ];
        let key_type = KeyType::Aes192;

        let expanded_key = key_expansion(key, key_type);
        let plaintext = cipher(message, &expanded_key, key_type);
        let plaintext = hex_array_to_string(&plaintext);

        println!("encrypt_192 = {plaintext}");
    }

    fn decrypt_256(key: &[u8; 32]) {
        let message = [
            0x47, 0xd5, 0xe9, 0x1a, 0x62, 0x3c, 0xfc, 0xec, 0xfd, 0x45, 0x1d, 0xb1, 0xa7, 0x67,
            0x1c, 0x41,
        ];
        let key_type = KeyType::Aes256;

        let expanded_key = key_expansion(key, key_type);
        let plaintext = inverse_cipher(message, &expanded_key, key_type);
        let plaintext = hex_array_to_string(&plaintext);

        println!("decrypt_256 = {plaintext}");
    }

    fn encrypt_256(key: &[u8; 32]) {
        let message = [
            0xfc, 0x5f, 0x6b, 0xd0, 0x04, 0x0b, 0x34, 0x02, 0xa5, 0x6c, 0xf4, 0x35, 0x5e, 0x63,
            0xa1, 0xe7,
        ];
        let key_type = KeyType::Aes256;

        let expanded_key = key_expansion(key, key_type);
        let plaintext = cipher(message, &expanded_key, key_type);
        let plaintext = hex_array_to_string(&plaintext);

        println!("encrypt_256 = {plaintext}");
    }
}

mod appendix_c {
    use std::env;

    use crate::cipher::cipher;
    use crate::constant::KeyType;
    use crate::debug;
    use crate::debug::hex_array_to_string;
    use crate::inverse_cipher::inverse_cipher;
    use crate::key_expansion::key_expansion;

    pub fn run() {
        let old_debug = env::var("DEBUG");

        env::set_var("DEBUG", "true");
        c_1();
        c_2();
        c_3();

        // reset DEBUG env variable
        match old_debug {
            Ok(value) => env::set_var("DEBUG", value),
            Err(_) => env::remove_var("DEBUG"),
        }
    }

    fn c_1() {
        println!("===== C.1 AES-128 =====");

        let plaintext: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        debug!(println!(
            "{:18} {}",
            "PLAINTEXT:",
            hex_array_to_string(&plaintext)
        ));

        let key: [u8; 16] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ];
        debug!(println!("{:18} {}", "KEY:", hex_array_to_string(&key)));
        debug!(println!());

        let key_type = KeyType::Aes128;

        let expanded_key = key_expansion(&key, key_type.to_owned());

        let ciphertext = cipher(plaintext, &expanded_key, key_type);

        debug!(println!(
            "{:18} {}",
            "CIPHERTEXT:",
            hex_array_to_string(&ciphertext)
        ));
        debug!(println!());

        let plaintext = inverse_cipher(ciphertext, &expanded_key, key_type);

        debug!(println!(
            "{:18} {}",
            "PLAINTEXT:",
            hex_array_to_string(&plaintext)
        ));
        debug!(println!());

        println!("=======================");
        println!();
    }

    fn c_2() {
        println!("===== C.2 AES-192 =====");

        let plaintext: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        debug!(println!(
            "{:18} {}",
            "PLAINTEXT:",
            hex_array_to_string(&plaintext)
        ));

        let key: [u8; 24] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
        ];
        debug!(println!("{:18} {}", "KEY:", hex_array_to_string(&key)));
        debug!(println!());

        let key_type = KeyType::Aes192;

        let expanded_key = key_expansion(&key, key_type.to_owned());

        let ciphertext = cipher(plaintext, &expanded_key, key_type);

        debug!(println!(
            "{:18} {}",
            "CIPHERTEXT:",
            hex_array_to_string(&ciphertext)
        ));
        debug!(println!());

        let plaintext = inverse_cipher(ciphertext, &expanded_key, key_type);

        debug!(println!(
            "{:18} {}",
            "PLAINTEXT:",
            hex_array_to_string(&plaintext)
        ));
        debug!(println!());

        println!("=======================");
        println!();
    }

    fn c_3() {
        println!("===== C.3 AES-256 =====");

        let plaintext: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        debug!(println!(
            "{:18} {}",
            "PLAINTEXT:",
            hex_array_to_string(&plaintext)
        ));

        let key: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        debug!(println!("{:18} {}", "KEY:", hex_array_to_string(&key)));
        debug!(println!());

        let key_type = KeyType::Aes256;

        let expanded_key = key_expansion(&key, key_type.to_owned());

        let ciphertext = cipher(plaintext, &expanded_key, key_type);

        debug!(println!(
            "{:18} {}",
            "CIPHERTEXT:",
            hex_array_to_string(&ciphertext)
        ));
        debug!(println!());

        let plaintext = inverse_cipher(ciphertext, &expanded_key, key_type);

        debug!(println!(
            "{:18} {}",
            "PLAINTEXT:",
            hex_array_to_string(&plaintext)
        ));
        debug!(println!());

        println!("=======================");
        println!();
    }
}
