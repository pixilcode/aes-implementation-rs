mod cipher;
mod constant;
mod debug;
mod finite_field;
mod inverse_cipher;
mod key_expansion;

fn main() {
    appendix_c::run();
}

mod appendix_c {
    use std::env;

    use crate::debug;
    use crate::debug::hex_array_to_string;
    use crate::constant::KeyType;
    use crate::key_expansion::key_expansion;
    use crate::cipher::cipher;

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
        let plaintext: [u8; 16] = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        debug!(println!("{:18} {}", "PLAINTEXT:", hex_array_to_string(&plaintext)));

        let key: [u8; 16] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f,
        ];
        debug!(println!("{:18} {}", "KEY:", hex_array_to_string(&key)));
        debug!(println!());

        let key_type = KeyType::Aes128;

        let expanded_key = key_expansion(&key, key_type.to_owned());

        let ciphertext = cipher(plaintext, &expanded_key, key_type);
        
        debug!(println!("{:18} {}", "CIPHERTEXT:", hex_array_to_string(&ciphertext)));
    }
    fn c_2() {}
    fn c_3() {}
}
