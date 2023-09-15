use crate::constant::{KeyType, S_BOX};
use crate::debug::{print_state, Step, print_key_sched, print_hex_array};
use crate::debug;
use crate::finite_field::FiniteField;

fn sub_bytes<S, T>(state: S) -> Vec<Vec<u8>>
where
    S: AsRef<[T]>,
    T: AsRef<[u8]>,
{
    state
        .as_ref()
        .iter()
        .map(|row| {
            row.as_ref()
                .iter()
                .map(|value| {
                    let row = (value >> 4) & 0b1111;
                    let column = value & 0b1111;
                    S_BOX[row as usize][column as usize]
                })
                .collect()
        })
        .collect()
}

fn shift_rows<S, T>(state: S) -> Vec<Vec<u8>>
where
    S: AsRef<[T]>,
    T: AsRef<[u8]>,
{
    let state = state.as_ref();

    state
        .as_ref()
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let row = row.as_ref();
            let mut new_row = vec![0; row.len()];

            for (j, value) in row.iter().enumerate() {
                let new_index = (j + row.len() - i) % row.len();
                new_row[new_index] = *value;
            }

            new_row
        })
        .collect()
}

fn mix_columns<S, T>(state: S) -> Vec<Vec<u8>>
where
    S: AsRef<[T]>,
    T: AsRef<[u8]>,
{
    let state: Vec<Vec<FiniteField>> = state
        .as_ref()
        .iter()
        .map(AsRef::as_ref)
        .map(|row| row.iter().map(Into::into).collect())
        .collect();
    let num_rows = state.len();
    let num_cols = state[0].len();

    let mut result: Vec<Vec<FiniteField>> = vec![vec![0.into(); num_cols]; num_rows];

    for c in 0..num_cols {
        result[0][c] =
            (state[0][c] * 0x02.into()) + (state[1][c] * 0x03.into()) + state[2][c] + state[3][c];
        result[1][c] =
            state[0][c] + (state[1][c] * 0x02.into()) + (state[2][c] * 0x03.into()) + state[3][c];
        result[2][c] =
            state[0][c] + state[1][c] + (state[2][c] * 0x02.into()) + (state[3][c] * 0x03.into());
        result[3][c] =
            (state[0][c] * 0x03.into()) + state[1][c] + state[2][c] + (state[3][c] * 0x02.into());
    }

    result
        .into_iter()
        .map(|row| row.into_iter().map(Into::into).collect())
        .collect()
}

pub fn get_key_sched(expanded_key: &[u32], round_key: usize, n_b: usize) -> &[u32] {
    let start_idx = round_key * n_b;
    &expanded_key[start_idx..(start_idx + n_b)]
}

pub fn add_round_key<S, T>(state: S, key_sched: &[u32]) -> Vec<Vec<u8>>
where
    S: AsRef<[T]>,
    T: AsRef<[u8]>,
{
    let state: Vec<Vec<u8>> = state
        .as_ref()
        .iter()
        .map(AsRef::as_ref)
        .map(|row| row.iter().copied().collect())
        .collect();
    let num_rows = state.len();
    let num_cols = state[0].len();

    let mut result: Vec<Vec<u8>> = vec![vec![0; num_cols]; num_rows];

    for c in 0..num_cols {
        let key_word = key_sched[c].to_be_bytes();

        result[0][c] = state[0][c] ^ key_word[0];
        result[1][c] = state[1][c] ^ key_word[1];
        result[2][c] = state[2][c] ^ key_word[2];
        result[3][c] = state[3][c] ^ key_word[3];
    }

    result
        .into_iter()
        .map(|row| row.into_iter().map(Into::into).collect())
        .collect()
}

pub fn cipher(input: impl AsRef<[u8]>, expanded_key: &[u32], key_type: KeyType) -> Vec<u8> {
    debug!(println!("CIPHER (ENCRYPT):"));

    debug!(print_hex_array(0, Step::Input, input.as_ref()));

    let n_b = key_type.n_b();
    let n_r = key_type.n_r();

    let state = input
        .as_ref()
        .into_iter()
        .enumerate()
        .fold(vec![], |mut result, (i, value)| {
            if i / n_b == 0 {
                result.push(vec![*value]);
            } else {
                let row = result.get_mut(i % n_b).unwrap();
                row.push(*value);
            }

            result
        });

    let key_sched = get_key_sched(expanded_key, 0, n_b);
    debug!(print_key_sched(0, Step::KeySchedule, key_sched));

    let state = add_round_key(state, key_sched);

    let state = (1..n_r).into_iter().fold(state, |state, round| {
        debug!(print_state(round, Step::Start, &state));

        let state = sub_bytes(state);
        debug!(print_state(round, Step::SubBytes, &state));

        let state = shift_rows(state);
        debug!(print_state(round, Step::ShiftRows, &state));

        let state = mix_columns(state);
        debug!(print_state(round, Step::MixColumns, &state));

        let key_sched = get_key_sched(expanded_key, round, n_b);
        debug!(print_key_sched(round, Step::KeySchedule, key_sched));

        let state = add_round_key(state, key_sched);

        state
    });

    let round = n_r;

    debug!(print_state(round, Step::Start, &state));

    let state = sub_bytes(state);
    debug!(print_state(round, Step::SubBytes, &state));

    let state = shift_rows(state);
    debug!(print_state(round, Step::ShiftRows, &state));

    let key_sched = get_key_sched(expanded_key, round, n_b);
    debug!(print_key_sched(round, Step::KeySchedule, key_sched));

    let state = add_round_key(state, key_sched);

    let mut result = Vec::with_capacity(state.len() * state[0].len());
    for i in 0..state[0].len() {
        for j in 0..state.len() {
            result.push(state[j][i]);
        }
    }

    debug!(print_hex_array(round, Step::Output, &result));
    debug!(println!());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_bytes_test() {
        let state = [
            [0x19, 0xa0, 0x9a, 0xe9],
            [0x3d, 0xf4, 0xc6, 0xf8],
            [0xe3, 0xe2, 0x8d, 0x48],
            [0xbe, 0x2b, 0x2a, 0x08],
        ];

        let state = sub_bytes(state);
        let sub = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30],
        ];
        assert_eq!(state, sub);
    }

    #[test]
    fn shift_rows_test() {
        let state = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30],
        ];

        let state = shift_rows(state);
        let shift = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5],
        ];
        assert_eq!(state, shift);
    }

    #[test]
    fn mix_columns_state() {
        let state = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5],
        ];

        let state = mix_columns(state);
        let mix = [
            [0x04, 0xe0, 0x48, 0x28],
            [0x66, 0xcb, 0xf8, 0x06],
            [0x81, 0x19, 0xd3, 0x26],
            [0xe5, 0x9a, 0x7a, 0x4c],
        ];
        assert_eq!(state, mix);
    }

    #[test]
    fn add_round_key_test() {
        let state = [
            [0x04, 0xe0, 0x48, 0x28],
            [0x66, 0xcb, 0xf8, 0x06],
            [0x81, 0x19, 0xd3, 0x26],
            [0xe5, 0x9a, 0x7a, 0x4c],
        ];

        let expanded_key: [u32; 44] = [
            0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0xa0fafe17, 0x88542cb1, 0x23a33939,
            0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
            0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
            0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
            0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
            0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
            0xe13f0cc8, 0xb6630ca6,
        ];

        let key_sched = get_key_sched(&expanded_key, 1, 4);
        let state = add_round_key(state, key_sched);
        let round = [
            [0xa4, 0x68, 0x6b, 0x02],
            [0x9c, 0x9f, 0x5b, 0x6a],
            [0x7f, 0x35, 0xea, 0x50],
            [0xf2, 0x2b, 0x43, 0x49],
        ];
        assert_eq!(state, round);
    }

    #[test]
    fn cipher_test() {
        let input = [
            0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37,
            0x07, 0x34,
        ];

        let expanded_key: [u32; 44] = [
            0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0xa0fafe17, 0x88542cb1, 0x23a33939,
            0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
            0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
            0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
            0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
            0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
            0xe13f0cc8, 0xb6630ca6,
        ];

        let expected: [u8; 16] = [
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32,
        ];

        let result: Vec<u8> = cipher(input, &expanded_key, KeyType::Aes128);

        for val in expected {
            print!("0x{:02x} ", val);
        }
        println!();
        for val in &result {
            print!("0x{:02x} ", val);
        }
        println!();

        assert_eq!(expected.to_vec(), result);
    }
}
