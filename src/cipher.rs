use crate::constant::S_BOX;
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

fn add_round_key<S, T>(state: S, expanded_key: &[u32], round_key: usize, n_b: usize) -> Vec<Vec<u8>>
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
        let key_word = expanded_key[(round_key * n_b) + c].to_be_bytes();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transformation_test() {
        let init_state = [
            [0x19, 0xa0, 0x9a, 0xe9],
            [0x3d, 0xf4, 0xc6, 0xf8],
            [0xe3, 0xe2, 0x8d, 0x48],
            [0xbe, 0x2b, 0x2a, 0x08],
        ];

        let state = sub_bytes(init_state);
        let sub = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0x27, 0xbf, 0xb4, 0x41],
            [0x11, 0x98, 0x5d, 0x52],
            [0xae, 0xf1, 0xe5, 0x30],
        ];
        assert_eq!(state, sub);

        let state = shift_rows(state);
        let shift = [
            [0xd4, 0xe0, 0xb8, 0x1e],
            [0xbf, 0xb4, 0x41, 0x27],
            [0x5d, 0x52, 0x11, 0x98],
            [0x30, 0xae, 0xf1, 0xe5],
        ];
        assert_eq!(state, shift);

        let state = mix_columns(state);
        let mix = [
            [0x04, 0xe0, 0x48, 0x28],
            [0x66, 0xcb, 0xf8, 0x06],
            [0x81, 0x19, 0xd3, 0x26],
            [0xe5, 0x9a, 0x7a, 0x4c],
        ];
        assert_eq!(state, mix);

        let expanded_key: [u32; 44] = [
            0x2b7e1516, 0x28aed2a6, 0xabf71588, 0x09cf4f3c, 0xa0fafe17, 0x88542cb1, 0x23a33939,
            0x2a6c7605, 0xf2c295f2, 0x7a96b943, 0x5935807a, 0x7359f67f, 0x3d80477d, 0x4716fe3e,
            0x1e237e44, 0x6d7a883b, 0xef44a541, 0xa8525b7f, 0xb671253b, 0xdb0bad00, 0xd4d1c6f8,
            0x7c839d87, 0xcaf2b8bc, 0x11f915bc, 0x6d88a37a, 0x110b3efd, 0xdbf98641, 0xca0093fd,
            0x4e54f70e, 0x5f5fc9f3, 0x84a64fb2, 0x4ea6dc4f, 0xead27321, 0xb58dbad2, 0x312bf560,
            0x7f8d292f, 0xac7766f3, 0x19fadc21, 0x28d12941, 0x575c006e, 0xd014f9a8, 0xc9ee2589,
            0xe13f0cc8, 0xb6630ca6,
        ];

        let state = add_round_key(state, &expanded_key, 1, 4);
        let round = [
            [0xa4, 0x68, 0x6b, 0x02],
            [0x9c, 0x9f, 0x5b, 0x6a],
            [0x7f, 0x35, 0xea, 0x50],
            [0xf2, 0x2b, 0x43, 0x49],
        ];
        assert_eq!(state, round);
    }
}
